multiversx_sc::imports!();

use crate::{
    basics::errors::{
        ERROR_BAD_PAYMENT_AMOUNT, ERROR_BAD_PAYMENT_TOKEN, ERROR_LS_TOKEN_NOT_ISSUED,
    },
    contexts::base::StorageCache,
    liquidity_pool, setup,
};

#[multiversx_sc::module]
pub trait DelegateVoteModule:
    setup::config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + liquidity_pool::LiquidityPoolModule
    + setup::vote::VoteModule
{
    #[payable("*")]
    #[endpoint(delegateVote)]
    fn delegate_vote(&self, proposal: ManagedBuffer, vote_type: ManagedBuffer) {
        self.blockchain().check_caller_is_user_account();
        let storage_cache = StorageCache::new(self);
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        require!(
            storage_cache.ls_token_id.is_valid_esdt_identifier(),
            ERROR_LS_TOKEN_NOT_ISSUED
        );
        require!(
            payment.token_identifier == storage_cache.ls_token_id,
            ERROR_BAD_PAYMENT_TOKEN
        );
        require!(payment.amount > 0, ERROR_BAD_PAYMENT_AMOUNT);

        let balance_to_vote = self.get_egld_amount(&payment.amount, &storage_cache);

        self.call_delegate_vote(proposal, vote_type, &caller, balance_to_vote);

        // we use the ls token only to access amount staked, afterwards we send it back to the caller
        self.send().direct_esdt(
            &caller,
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
    }

    fn call_delegate_vote(
        &self,
        proposal: ManagedBuffer,
        vote_type: ManagedBuffer,
        delegate_to: &ManagedAddress,
        rewards_reserve: BigUint,
    ) {
        let governance_contract = self.get_governance_sc();

        self.tx()
            .to(governance_contract.clone())
            .typed(GovernanceSCProxy)
            .delegate_vote(proposal, vote_type, delegate_to, rewards_reserve)
            .sync_call();
    }
}
