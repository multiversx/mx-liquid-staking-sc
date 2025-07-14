multiversx_sc::imports!();

use crate::{
    basics::{
        self,
        constants::TEN_DAYS,
        errors::{ERROR_BAD_PAYMENT_TOKEN, ERROR_LS_TOKEN_NOT_ISSUED},
    },
    contexts::base::StorageCache,
    liquidity_pool,
    setup::{self, vote::LockedFunds},
};

#[multiversx_sc::module]
pub trait DelegateVoteModule:
    setup::config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + liquidity_pool::LiquidityPoolModule
    + setup::vote::VoteModule
    + basics::events::EventsModule
{
    #[payable("*")]
    #[endpoint(delegateVote)]
    fn delegate_vote(&self, proposal: ManagedBuffer, vote_type: ManagedBuffer) {
        self.blockchain().check_caller_is_user_account();
        let caller = self.blockchain().get_caller();
        require!(
            !self.voted_proposals(&caller).contains(&proposal),
            "already voted for this proposal"
        );

        let balance_to_vote = self.handle_balance_to_vote(&caller);
        self.call_delegate_vote(&proposal, vote_type, &caller, &balance_to_vote);

        self.voted_proposals(&caller).insert(proposal);
    }

    fn handle_balance_to_vote(&self, caller: &ManagedAddress) -> BigUint {
        let storage_cache = StorageCache::new(self);

        require!(
            storage_cache.ls_token_id.is_valid_esdt_identifier(),
            ERROR_LS_TOKEN_NOT_ISSUED
        );

        let payments = self.call_value().all_transfers();
        require!(
            payments.len() == 1     // user comes with new payment
                || (payments.is_empty() && !self.locked_vote_balance(caller).is_empty()), // user already has locked LS tokens from a previous proposal vote
            "invalid payment or missing voting power"
        );

        // the require above ensures us that at least in 1 place from below to_be_locked_balance will be increased

        let claim_back = self.blockchain().get_block_timestamp() + TEN_DAYS;
        let mut to_be_locked_balance = LockedFunds {
            funds: EsdtTokenPayment::new(storage_cache.ls_token_id.clone(), 0, BigUint::zero()),
            claim_back,
        };

        if !self.locked_vote_balance(caller).is_empty() {
            let locked_balance = self.locked_vote_balance(caller).get();
            to_be_locked_balance.funds = locked_balance.funds;
        }

        if !payments.is_empty() {
            let payment = payments.get(0).clone().unwrap_esdt();
            require!(
                payment.token_identifier == storage_cache.ls_token_id,
                ERROR_BAD_PAYMENT_TOKEN
            );
            to_be_locked_balance.funds.amount += payment.amount;
        }

        let egld_amount = self.get_egld_amount(&to_be_locked_balance.funds.amount, &storage_cache);
        self.locked_vote_balance(caller).set(to_be_locked_balance);
        self.tokens_locked_for_delegate_vote_event(&caller, &egld_amount, claim_back);
        egld_amount
    }

    fn call_delegate_vote(
        &self,
        proposal: &ManagedBuffer,
        vote_type: ManagedBuffer,
        delegate_to: &ManagedAddress,
        rewards_reserve: &BigUint,
    ) {
        let governance_contract = self.get_governance_sc();

        self.tx()
            .to(governance_contract.clone())
            .typed(GovernanceSCProxy)
            .delegate_vote(proposal, vote_type, delegate_to, rewards_reserve)
            .sync_call();
    }
}
