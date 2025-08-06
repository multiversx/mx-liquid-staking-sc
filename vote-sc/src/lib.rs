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
        proof: ManagedVec<Hash<Self::Api>>,
    ) {
        let caller = self.blockchain().get_caller();
        require!(!self.liquid_staking_sc().is_empty(), LS_SC_NOT_SET);
        self.require_caller_not_self(&caller);
        self.check_caller_has_power(&caller, proposal_id, &voting_power, proof);

        let ls_sc_address = self.liquid_staking_sc().get();
        let gas_for_async_call = self.get_gas_for_sync_call();
        self.tx()
            .to(ls_sc_address)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .delegate_vote(proposal_id, vote, caller, voting_power)
            .gas(gas_for_async_call)
            .sync_call();
    }

    fn check_caller_has_power(
        &self,
        caller: &ManagedAddress,
        proposal_id: ProposalId,
        voting_power: &BigUint<Self::Api>,
        proof: ManagedVec<ManagedByteArray<HASH_LENGTH>>,
    ) {
        let wrapped_root_hash = self.get_root_hash(proposal_id);

        if let OptionalValue::Some(root_hash) = wrapped_root_hash {
            self.verify_merkle_proof(caller, voting_power, proof, root_hash);
        } else {
            sc_panic!(INVALID_MERKLE_PROOF);
        }
    }

    fn get_gas_for_sync_call(&self) -> u64 {
        let gas_left = self.blockchain().get_gas_left();
        require!(
            gas_left > MIN_GAS_FOR_SYNC_CALL + MIN_GAS_FINISH_EXEC,
            ERROR_INSUFFICIENT_GAS_FOR_SYNC
        );
        gas_left - MIN_GAS_FINISH_EXEC
    }

    #[view]
    #[storage_mapper("liquidStakingAddress")]
    fn liquid_staking_sc(&self) -> SingleValueMapper<ManagedAddress>;
}
