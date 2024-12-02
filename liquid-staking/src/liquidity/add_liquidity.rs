multiversx_sc::imports!();

use crate::{
    basics,
    basics::constants::{MINIMUM_LIQUIDITY, MIN_EGLD_TO_DELEGATE, MIN_GAS_FOR_CALLBACK},
    basics::errors::{
        ERROR_BAD_PAYMENT_AMOUNT, ERROR_DELEGATION_CONTRACT_NOT_INITIALIZED, ERROR_NOT_ACTIVE,
    },
    config, delegation, delegation_proxy, liquidity_pool, StorageCache,
};

#[multiversx_sc::module]
pub trait AddLiquidityModule:
    config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + delegation::DelegationModule
    + liquidity_pool::LiquidityPoolModule
    + basics::events::EventsModule
{
    #[payable("EGLD")]
    #[endpoint(addLiquidity)]
    fn add_liquidity(&self) {
        self.blockchain().check_caller_is_user_account();
        let storage_cache = StorageCache::new(self);
        let caller = self.blockchain().get_caller();

        let payment = self.call_value().egld_value().clone_value();
        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        if storage_cache.ls_token_supply == 0 {
            require!(
                caller == self.blockchain().get_owner_address(),
                ERROR_DELEGATION_CONTRACT_NOT_INITIALIZED
            );
        }
        require!(payment > MIN_EGLD_TO_DELEGATE, ERROR_BAD_PAYMENT_AMOUNT);

        self.call_delegate(caller, payment);
    }

    #[promises_callback]
    fn add_liquidity_callback(
        &self,
        caller: ManagedAddress,
        delegation_contract: ManagedAddress,
        staked_tokens: BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.delegation_contract_data(&delegation_contract)
                    .update(|contract_data| {
                        contract_data.total_staked_from_ls_contract += &staked_tokens;
                    });

                let mut storage_cache = StorageCache::new(self);
                let mut ls_token_amount_before_add = BigUint::zero();
                if storage_cache.ls_token_supply == 0 {
                    ls_token_amount_before_add += MINIMUM_LIQUIDITY;
                }

                let ls_token_amount = self.pool_add_liquidity(&staked_tokens, &mut storage_cache)
                    - ls_token_amount_before_add;
                let user_payment = self.mint_ls_token(ls_token_amount);
                self.send().direct_esdt(
                    &caller,
                    &user_payment.token_identifier,
                    user_payment.token_nonce,
                    &user_payment.amount,
                );

                self.emit_add_liquidity_event(&storage_cache, &caller, user_payment.amount);
            }
            ManagedAsyncCallResult::Err(_) => {
                self.send().direct_egld(&caller, &staked_tokens);
                self.move_delegation_contract_to_back(delegation_contract);
            }
        }
    }

    fn call_delegate(&self, caller: ManagedAddress, payment: BigUint) {
        let delegation_contract = self.get_delegation_contract_for_delegate(&payment);

        let gas_for_async_call = self.get_gas_for_async_call();
        self.tx()
            .to(delegation_contract.clone())
            .typed(delegation_proxy::DelegationMockProxy)
            .delegate()
            .egld(payment.clone())
            .gas(gas_for_async_call)
            .callback(AddLiquidityModule::callbacks(self).add_liquidity_callback(
                caller,
                delegation_contract,
                payment,
            ))
            .gas_for_callback(MIN_GAS_FOR_CALLBACK)
            .register_promise();
    }
}
