#![no_std]

use multiversx_sc::imports::*;
pub mod vote_type;
pub use vote_type::VoteType;

#[multiversx_sc::contract]
pub trait VoteMock {
    #[init]
    fn init(&self) {}

    #[endpoint(delegationVote)]
    fn delegate_vote(
        &self,
        proposal: usize,
        vote_type: VoteType,
        delegate_to: ManagedAddress,
        amount: BigUint,
    ) {
        /* todo */
    }
}
