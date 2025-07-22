multiversx_sc::imports!();

use crate::{
    basics::errors::{ERROR_ALREADY_VOTED, ERROR_INVALID_CALLER, ERROR_VOTE_SC_NOT_SET},
    setup::governance,
};
#[multiversx_sc::module]
pub trait VoteModule:
    governance::GovernanceModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[endpoint(delegateVote)]
    fn delegate_vote(
        &self,
        proposal: u32,
        vote_type: ManagedBuffer,
        delegate_to: ManagedAddress,
        voting_power: BigUint,
    ) {
        self.blockchain().check_caller_is_user_account();
        let caller = self.blockchain().get_caller();

        require!(!self.vote_contract().is_empty(), ERROR_VOTE_SC_NOT_SET);
        let vote_sc = self.vote_contract().get();
        require!(caller == vote_sc, ERROR_INVALID_CALLER);
        require!(
            !self.voted_proposals(&caller).contains(&proposal),
            ERROR_ALREADY_VOTED
        );

        self.call_delegate_vote(proposal, vote_type, &delegate_to, &voting_power);
        self.voted_proposals(&delegate_to).insert(proposal);
    }

    fn call_delegate_vote(
        &self,
        proposal: u32,
        vote_type: ManagedBuffer,
        delegate_to: &ManagedAddress,
        voting_power: &BigUint,
    ) {
        let governance_contract = self.get_governance_sc();

        self.tx()
            .to(governance_contract.clone())
            .typed(GovernanceSCProxy)
            .delegate_vote(proposal, vote_type, delegate_to, voting_power)
            .async_call_and_exit();
    }

    #[storage_mapper("voteContract")]
    fn vote_contract(&self) -> SingleValueMapper<ManagedAddress>;
}
