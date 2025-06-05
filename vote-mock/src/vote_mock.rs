#![no_std]

use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;
pub mod vote_type;
pub use vote_type::VoteType;

type Epoch = u64;

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Eq, Debug, ManagedVecItem,
)]
pub struct Proposal<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub start_vote_epoch: Epoch,
    pub end_vote_epoch: Epoch,
}

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Eq, Debug, ManagedVecItem,
)]
pub struct PoweredVotes<M: ManagedTypeApi> {
    pub yes: BigUint<M>,
    pub no: BigUint<M>,
    pub veto: BigUint<M>,
    pub abstain: BigUint<M>,
}

#[multiversx_sc::contract]
pub trait VoteMock {
    #[init]
    fn init(&self, ls_contract: ManagedAddress) {
        self.ls_contract().set(ls_contract);
    }

    #[endpoint]
    fn propose(&self, name: ManagedBuffer, start_vote_epoch: Epoch, end_vote_epoch: Epoch) {
        let current_epoch = self.blockchain().get_block_epoch();
        require!(
            start_vote_epoch >= current_epoch,
            "Starting period cannot be in the past"
        );
        require!(
            end_vote_epoch > start_vote_epoch,
            "Ending voting period has to be after the starting one"
        );
        let new_proposal = Proposal {
            name,
            start_vote_epoch,
            end_vote_epoch,
        };
        let proposal_id = self.proposals().push(&new_proposal);

        self.votes(proposal_id).set(PoweredVotes {
            yes: BigUint::zero(),
            no: BigUint::zero(),
            veto: BigUint::zero(),
            abstain: BigUint::zero(),
        });
    }

    #[endpoint(delegationVote)]
    fn delegate_vote(
        &self,
        proposal_id: usize,
        vote_type: VoteType,
        delegate_to: ManagedAddress,
        power: BigUint,
    ) {
        self.require_caller_is_ls_sc();
        self.require_porposal_active(proposal_id);
        require!(
            !self.user_votes(&delegate_to).contains(&proposal_id),
            "Already voted for proposal"
        );
        self.user_votes(&delegate_to).insert(proposal_id);
        self.votes(proposal_id).update(|votes| match vote_type {
            VoteType::Yes => votes.yes += power,
            VoteType::No => votes.no += power,
            VoteType::Veto => votes.veto += power,
            VoteType::Abstain => votes.abstain += power,
        })
    }

    fn require_caller_is_ls_sc(&self) {
        let ls_contract = self.ls_contract().get();
        let caller = self.blockchain().get_caller();
        require!(ls_contract == caller, "Only ls contract can call this");
    }

    fn require_porposal_active(&self, proposal_id: usize) {
        let proposal = self.proposals().get(proposal_id);
        let current_epoch = self.blockchain().get_block_epoch();
        require!(
            current_epoch >= proposal.start_vote_epoch && current_epoch < proposal.end_vote_epoch,
            "proposal is not active"
        )
    }

    #[storage_mapper("lsContract")]
    fn ls_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("proposals")]
    fn proposals(&self) -> VecMapper<Proposal<Self::Api>>;

    #[storage_mapper("votes")]
    fn votes(&self, proposal_id: usize) -> SingleValueMapper<PoweredVotes<Self::Api>>;

    #[storage_mapper("userVotes")]
    fn user_votes(&self, user: &ManagedAddress) -> UnorderedSetMapper<usize>;
}
