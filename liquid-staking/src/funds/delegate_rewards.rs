multiversx_sc::imports!();

use crate::{
    basics,
    basics::constants::{MIN_EGLD_TO_DELEGATE, MIN_GAS_FOR_CALLBACK},
    basics::errors::{ERROR_BAD_DELEGATION_AMOUNT, ERROR_CLAIM_REDELEGATE, ERROR_NOT_ACTIVE},
    config,
    delegation::{self, ClaimStatusType},
    delegation_proxy, StorageCache,
};

#[multiversx_sc::module]
pub trait DelegateRewardsModule:
    config::ConfigModule
    + delegation::DelegationModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + basics::events::EventsModule
{
    #[endpoint(delegateRewards)]
    fn delegate_rewards(&self) {
        let storage_cache = StorageCache::new(self);
        let claim_status = self.delegation_claim_status().get();
        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        require!(
            claim_status.status == ClaimStatusType::Delegable,
            ERROR_CLAIM_REDELEGATE
        );

        let rewards_reserve = self.rewards_reserve().take();

        require!(
            rewards_reserve >= MIN_EGLD_TO_DELEGATE,
            ERROR_BAD_DELEGATION_AMOUNT
        );

        let delegation_contract = self.get_delegation_contract_for_delegate(&rewards_reserve);

        let gas_for_async_call = self.get_gas_for_async_call();
        self.tx()
            .to(delegation_contract.clone())
            .typed(delegation_proxy::DelegationMockProxy)
            .delegate()
            .egld(rewards_reserve.clone())
            .gas(gas_for_async_call)
            .callback(
                DelegateRewardsModule::callbacks(self)
                    .delegate_rewards_callback(delegation_contract, rewards_reserve),
            )
            .gas_for_callback(MIN_GAS_FOR_CALLBACK)
            .register_promise();
    }

    #[promises_callback]
    fn delegate_rewards_callback(
        &self,
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

                self.delegation_claim_status()
                    .update(|claim_status| claim_status.status = ClaimStatusType::Redelegated);

                let mut storage_cache = StorageCache::new(self);
                storage_cache.virtual_egld_reserve += &staked_tokens;
                let sc_address = self.blockchain().get_sc_address();

                self.emit_add_liquidity_event(&storage_cache, &sc_address, BigUint::zero());
            }
            ManagedAsyncCallResult::Err(_) => {
                self.move_delegation_contract_to_back(delegation_contract);
                self.rewards_reserve()
                    .update(|value| *value += staked_tokens)
            }
        }
    }
}
