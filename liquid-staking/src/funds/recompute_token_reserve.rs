multiversx_sc::imports!();

use crate::{
    basics::constants::{MIN_EGLD_TO_DELEGATE, RECOMPUTE_BLOCK_OFFSET},
    basics::errors::{ERROR_NOT_ACTIVE, ERROR_RECOMPUTE_RESERVES, ERROR_RECOMPUTE_TOO_SOON},
    setup::{self, delegation::ClaimStatusType},
    StorageCache,
};

#[multiversx_sc::module]
pub trait RecomputeTokenReserveModule:
    setup::config::ConfigModule
    + setup::delegation::DelegationModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[endpoint(recomputeTokenReserve)]
    fn recompute_token_reserve(&self) {
        let storage_cache = StorageCache::new(self);
        let claim_status_mapper = self.delegation_claim_status();
        let mut claim_status = claim_status_mapper.get();

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        require!(
            claim_status.status == ClaimStatusType::Finished,
            ERROR_RECOMPUTE_RESERVES
        );

        let current_block = self.blockchain().get_block_nonce();
        require!(
            current_block >= claim_status.last_claim_block + RECOMPUTE_BLOCK_OFFSET,
            ERROR_RECOMPUTE_TOO_SOON
        );

        if self.rewards_reserve().get() >= MIN_EGLD_TO_DELEGATE {
            claim_status.status = ClaimStatusType::Delegable;
        } else {
            claim_status.status = ClaimStatusType::Insufficient;
        }

        claim_status_mapper.set(claim_status);
    }
}
