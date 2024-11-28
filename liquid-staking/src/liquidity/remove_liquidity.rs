multiversx_sc::imports!();

use crate::{
    basics,
    basics::constants::{MIN_EGLD_TO_DELEGATE, MIN_GAS_FOR_CALLBACK},
    basics::errors::{
        ERROR_BAD_PAYMENT_AMOUNT, ERROR_BAD_PAYMENT_TOKEN, ERROR_INSUFFICIENT_UNSTAKE_AMOUNT,
        ERROR_LS_TOKEN_NOT_ISSUED, ERROR_NOT_ACTIVE,
    },
    config::{self, UnstakeTokenAttributes, UNBOND_PERIOD},
    delegation, delegation_proxy, liquidity_pool, StorageCache,
};

#[multiversx_sc::module]
pub trait RemoveLiquidityModule:
    config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + delegation::DelegationModule
    + liquidity_pool::LiquidityPoolModule
    + basics::events::EventsModule
{
    #[payable("*")]
    #[endpoint(removeLiquidity)]
    fn remove_liquidity(&self) {
        self.blockchain().check_caller_is_user_account();
        let mut storage_cache = StorageCache::new(self);
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        require!(
            storage_cache.ls_token_id.is_valid_esdt_identifier(),
            ERROR_LS_TOKEN_NOT_ISSUED
        );
        require!(
            payment.token_identifier == storage_cache.ls_token_id,
            ERROR_BAD_PAYMENT_TOKEN
        );
        require!(payment.amount > 0, ERROR_BAD_PAYMENT_AMOUNT);

        let egld_to_unstake = self.pool_remove_liquidity(&payment.amount, &mut storage_cache);
        require!(
            egld_to_unstake >= MIN_EGLD_TO_DELEGATE,
            ERROR_INSUFFICIENT_UNSTAKE_AMOUNT
        );
        self.burn_ls_token(&payment.amount);

        let delegation_contract = self.get_delegation_contract_for_undelegate(&egld_to_unstake);

        let delegation_contract_mapper = self.delegation_contract_data(&delegation_contract);
        delegation_contract_mapper
            .update(|contract_data| contract_data.egld_in_ongoing_undelegation += &egld_to_unstake);

        let gas_for_async_call = self.get_gas_for_async_call();
        self.tx()
            .to(delegation_contract.clone())
            .typed(delegation_proxy::DelegationMockProxy)
            .undelegate(egld_to_unstake.clone())
            .gas(gas_for_async_call)
            .callback(
                RemoveLiquidityModule::callbacks(self).remove_liquidity_callback(
                    caller,
                    delegation_contract,
                    egld_to_unstake,
                    payment.amount,
                ),
            )
            .gas_for_callback(MIN_GAS_FOR_CALLBACK)
            .register_promise();
    }

    #[promises_callback]
    fn remove_liquidity_callback(
        &self,
        caller: ManagedAddress,
        delegation_contract: ManagedAddress,
        egld_to_unstake: BigUint,
        ls_tokens_to_be_burned: BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let mut storage_cache = StorageCache::new(self);
        let delegation_contract_mapper = self.delegation_contract_data(&delegation_contract);

        delegation_contract_mapper.update(|contract_data| {
            contract_data.egld_in_ongoing_undelegation -= &egld_to_unstake;
        });

        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let current_epoch = self.blockchain().get_block_epoch();
                let unbond_epoch = current_epoch + UNBOND_PERIOD;

                delegation_contract_mapper.update(|contract_data| {
                    contract_data.total_staked_from_ls_contract -= &egld_to_unstake;
                    contract_data.total_unstaked_from_ls_contract += &egld_to_unstake;
                });

                let virtual_position = UnstakeTokenAttributes {
                    delegation_contract,
                    unstake_epoch: current_epoch,
                    unstake_amount: egld_to_unstake,
                    unbond_epoch,
                };

                let user_payment = self.mint_unstake_tokens(&virtual_position);
                self.send().direct_esdt(
                    &caller,
                    &user_payment.token_identifier,
                    user_payment.token_nonce,
                    &user_payment.amount,
                );

                self.emit_remove_liquidity_event(
                    &storage_cache,
                    ls_tokens_to_be_burned,
                    user_payment.amount,
                );
            }
            ManagedAsyncCallResult::Err(_) => {
                let ls_token_amount = self.pool_add_liquidity(&egld_to_unstake, &mut storage_cache);
                let user_payment = self.mint_ls_token(ls_token_amount);
                self.send().direct_esdt(
                    &caller,
                    &user_payment.token_identifier,
                    user_payment.token_nonce,
                    &user_payment.amount,
                );
                self.move_delegation_contract_to_back(delegation_contract);
            }
        }
    }
}
