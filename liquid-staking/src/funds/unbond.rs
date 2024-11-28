multiversx_sc::imports!();

use crate::{
    basics::errors::{
        ERROR_BAD_PAYMENT_AMOUNT, ERROR_BAD_PAYMENT_TOKEN, ERROR_NOT_ACTIVE,
        ERROR_UNSTAKE_PERIOD_NOT_PASSED,
    },
    config::{self, UnstakeTokenAttributes},
    delegation, liquidity_pool, StorageCache,
};

#[multiversx_sc::module]
pub trait UnbondModule:
    config::ConfigModule
    + delegation::DelegationModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + liquidity_pool::LiquidityPoolModule
{
    #[payable("*")]
    #[endpoint(unbondTokens)]
    fn unbond_tokens(&self) {
        self.blockchain().check_caller_is_user_account();
        let storage_cache = StorageCache::new(self);
        let payment = self.call_value().single_esdt();
        let caller = self.blockchain().get_caller();

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );

        require!(
            payment.token_identifier == self.unstake_token().get_token_id(),
            ERROR_BAD_PAYMENT_TOKEN
        );
        require!(payment.amount > 0, ERROR_BAD_PAYMENT_AMOUNT);

        let mut total_unstake_amount = BigUint::zero();

        let unstake_token_attributes: UnstakeTokenAttributes<Self::Api> = self
            .unstake_token()
            .get_token_attributes(payment.token_nonce);

        let current_epoch = self.blockchain().get_block_epoch();
        require!(
            current_epoch >= unstake_token_attributes.unbond_epoch,
            ERROR_UNSTAKE_PERIOD_NOT_PASSED
        );

        let delegation_contract = unstake_token_attributes.delegation_contract.clone();
        let unstake_amount = unstake_token_attributes.unstake_amount.clone();
        let delegation_contract_mapper = self.delegation_contract_data(&delegation_contract);
        let delegation_contract_data = delegation_contract_mapper.get();

        require!(
            delegation_contract_data.total_unbonded_from_ls_contract >= unstake_amount,
            "Nothing to unbond"
        );
        delegation_contract_mapper.update(|contract_data| {
            contract_data.total_unstaked_from_ls_contract -= &unstake_amount;
            contract_data.total_unbonded_from_ls_contract -= &unstake_amount
        });

        total_unstake_amount += unstake_amount;
        self.burn_unstake_tokens(payment.token_nonce);
        self.send().direct_egld(&caller, &total_unstake_amount);
    }
}
