multiversx_sc::imports!();

use crate::{
    basics::constants::MIN_GAS_FOR_CALLBACK,
    basics::errors::ERROR_NOT_ACTIVE,
    config::{self},
    delegation, delegation_proxy, StorageCache,
};

#[multiversx_sc::module]
pub trait WithdrawModule:
    config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + delegation::DelegationModule
{
    #[endpoint(withdrawAll)]
    fn withdraw_all(&self, delegation_contract: ManagedAddress) {
        self.blockchain().check_caller_is_user_account();
        let storage_cache = StorageCache::new(self);

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );

        let gas_for_async_call = self.get_gas_for_async_call();
        self.tx()
            .to(delegation_contract.clone())
            .typed(delegation_proxy::DelegationMockProxy)
            .withdraw()
            .gas(gas_for_async_call)
            .callback(WithdrawModule::callbacks(self).withdraw_tokens_callback(delegation_contract))
            .gas_for_callback(MIN_GAS_FOR_CALLBACK)
            .register_promise();
    }

    #[promises_callback]
    fn withdraw_tokens_callback(
        &self,
        provider: ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let withdraw_amount = self.call_value().egld_value().clone_value();
                let delegation_contract_mapper = self.delegation_contract_data(&provider);
                if withdraw_amount > 0u64 {
                    delegation_contract_mapper.update(|contract_data| {
                        contract_data.total_unbonded_from_ls_contract += &withdraw_amount
                    });
                }
            }
            ManagedAsyncCallResult::Err(_) => {}
        }
    }
}
