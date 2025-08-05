#![no_std]

multiversx_sc::imports!();

mod caller_check;
pub mod constants;
mod errors;
pub mod events;
pub mod liquid_staking_proxy;
pub mod views;

use crate::{constants::*, errors::*};

#[multiversx_sc::contract]
pub trait VoteSC:
    caller_check::CallerCheckModule + events::EventsModule + views::ViewsModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint]
    fn set_root_hash(&self, root_hash: Hash<Self::Api>, proposal_id: ProposalId) {
        require!(!root_hash.is_empty(), INVALID_ROOT_HASH);
        self.proposal_root_hash(proposal_id).set(root_hash)
    }

    #[only_owner]
    #[endpoint]
    fn set_liquid_staking_address(&self, address: ManagedAddress) {
        require!(
            self.blockchain().is_smart_contract(&address),
            INVALID_SC_ADDRESS
        );
        self.liquid_staking_sc().set(address);
    }

    #[endpoint]
    fn delegate_vote(
        &self,
        proposal_id: ProposalId,
        vote: ManagedBuffer,
        voting_power: BigUint<Self::Api>,
        proof: ArrayVec<ManagedByteArray<HASH_LENGTH>, PROOF_LENGTH>,
    ) {
        require!(!self.liquid_staking_sc().is_empty(), LS_SC_NOT_SET);
        self.require_caller_not_self();

        require!(
            self.confirm_voting_power(proposal_id, voting_power.clone(), proof),
            INVALID_MERKLE_PROOF
        );

        let voter = self.blockchain().get_caller();
        let ls_sc_address = self.liquid_staking_sc().get();
        self.tx()
            .to(ls_sc_address)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .delegate_vote(proposal_id, vote, voter, voting_power)
            .sync_call();
    }

    // delete this when done testing
    #[endpoint]
    fn delegate_test_vote(
        &self,
        _proposal_id: ProposalId,
        _vote: ManagedBuffer,
        _voting_power: BigUint<Self::Api>,
    ) {
        require!(!self.liquid_staking_sc().is_empty(), LS_SC_NOT_SET);
        self.require_caller_not_self();

        let _voter = self.blockchain().get_caller();
        let _ls_sc_address = self.liquid_staking_sc().get();
        // self.tx()
        //     .to(ls_sc_address)
        //     .typed(liquid_staking_proxy::LiquidStakingProxy)
        //     .delegate_vote(proposal_id, vote, voter, voting_power)
        //     .sync_call();
    }

    #[view]
    #[storage_mapper("liquidStakingAddress")]
    fn liquid_staking_sc(&self) -> SingleValueMapper<ManagedAddress>;
}
