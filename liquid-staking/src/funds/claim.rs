multiversx_sc::imports!();

use crate::{
    basics::{
        constants::{DEFAULT_GAS_TO_CLAIM_REWARDS, DEFAULT_MIN_GAS_TO_SAVE_PROGRESS},
        errors::{ERROR_NOT_ACTIVE, ERROR_NO_DELEGATION_CONTRACTS},
        events,
    },
    setup::{self, delegation::ClaimStatusType},
    StorageCache,
};

#[multiversx_sc::module]
pub trait ClaimModule:
    setup::config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + setup::delegation::DelegationModule
    + events::EventsModule
{
    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        let storage_cache = StorageCache::new(self);

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );

        let delegation_addresses_mapper = self.delegation_addresses_list();
        require!(
            !delegation_addresses_mapper.is_empty(),
            ERROR_NO_DELEGATION_CONTRACTS
        );
        let claim_status_mapper = self.delegation_claim_status();
        let old_claim_status = claim_status_mapper.get();
        let current_epoch = self.blockchain().get_block_epoch();

        self.check_claim_operation(old_claim_status, current_epoch);
        let mut delegation_addresses = self.addresses_to_claim();
        if delegation_addresses.is_empty() {
            self.prepare_claim_operation();
        }

        while !delegation_addresses.is_empty() {
            let gas_left = self.blockchain().get_gas_left();
            if gas_left < DEFAULT_MIN_GAS_TO_SAVE_PROGRESS {
                break;
            }

            let current_node = delegation_addresses.pop_back().unwrap();
            let address = current_node.clone().into_value();

            self.tx()
                .to(address.clone())
                .typed(DelegationSCProxy)
                .claim_rewards()
                .gas(DEFAULT_GAS_TO_CLAIM_REWARDS)
                .callback(ClaimModule::callbacks(self).claim_rewards_callback(address))
                .register_promise();

            delegation_addresses.remove_node(&current_node);
        }

        if delegation_addresses.is_empty() {
            claim_status_mapper.update(|claim_status| {
                claim_status.status = ClaimStatusType::Finished;
                claim_status.last_claim_block = self.blockchain().get_block_nonce();
                claim_status.last_claim_epoch = self.blockchain().get_block_epoch();
            });
        }
    }

    #[promises_callback]
    fn claim_rewards_callback(
        &self,
        delegation_address: ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(_) => {
                let payment = self.call_value().egld().clone_value();
                self.rewards_reserve().update(|value| *value += &payment);
                self.successful_claim_event(payment, &delegation_address);
            }
            ManagedAsyncCallResult::Err(_) => {
                self.failed_claim_event(&delegation_address);
            }
        }
    }
}
