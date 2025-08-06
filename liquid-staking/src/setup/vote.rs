multiversx_sc::imports!();

use crate::basics::{
    constants::{GasLimit, MIN_GAS_FINISH_EXEC, MIN_GAS_FOR_ASYNC_CALL},
    errors::{
        ERROR_ALREADY_VOTED, ERROR_INSUFFICIENT_GAS_FOR_ASYNC, ERROR_INVALID_CALLER,
        ERROR_INVALID_SC_ADDRESS, ERROR_VOTE_SC_NOT_SET,
    },
};
#[multiversx_sc::module]
pub trait VoteModule:
    multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[only_owner]
    #[endpoint]
    fn set_vote_contract(&self, sc_address: ManagedAddress) {
        self.require_sc_address(&sc_address);
        self.vote_contract().set(sc_address);
    }

    #[endpoint(delegateVote)]
    fn delegate_vote(
        &self,
        proposal: u32,
        vote_type: ManagedBuffer,
        delegate_to: ManagedAddress,
        voting_power: BigUint,
    ) {
        let caller = self.blockchain().get_caller();

        self.check_caller_is_vote_contract(&caller);
        require!(
            !self.voted_proposals(&caller).contains(&proposal),
            ERROR_ALREADY_VOTED
        );

        self.call_delegate_vote(proposal, vote_type, &delegate_to, &voting_power);
        self.voted_proposals(&delegate_to).insert(proposal);
    }

    fn check_caller_is_vote_contract(&self, caller: &ManagedAddress) {
        let vote_contract_mapper = self.vote_contract();
        require!(!vote_contract_mapper.is_empty(), ERROR_VOTE_SC_NOT_SET);

        let vote_sc = vote_contract_mapper.get();
        require!(caller == &vote_sc, ERROR_INVALID_CALLER);
    }

    fn call_delegate_vote(
        &self,
        proposal: u32,
        vote_type: ManagedBuffer,
        delegate_to: &ManagedAddress,
        voting_power: &BigUint,
    ) {
        let gas = self.get_gas_for_async_call();
        self.tx()
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .delegate_vote(proposal, vote_type, delegate_to, voting_power)
            .gas(gas)
            .register_promise();
    }

    fn require_sc_address(&self, address: &ManagedAddress) {
        require!(
            !address.is_zero() && self.blockchain().is_smart_contract(address),
            ERROR_INVALID_SC_ADDRESS
        );
    }

    fn get_gas_for_async_call(&self) -> GasLimit {
        let gas_left = self.blockchain().get_gas_left();
        require!(
            gas_left > MIN_GAS_FOR_ASYNC_CALL + MIN_GAS_FINISH_EXEC,
            ERROR_INSUFFICIENT_GAS_FOR_ASYNC
        );
        gas_left - MIN_GAS_FINISH_EXEC
    }

    #[view]
    #[storage_mapper("voteContract")]
    fn vote_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("votedProposals")]
    fn voted_proposals(&self, address: &ManagedAddress) -> UnorderedSetMapper<u32>;
}
