multiversx_sc::imports!();

use crate::{
    constants::{Hash, ProposalId, HASH_LENGTH},
    errors::NO_PROPOSAL,
};

#[multiversx_sc::module]
pub trait ViewsModule {
    #[view(getProposalRootHash)]
    fn get_root_hash(
        &self,
        proposal_id: ProposalId,
    ) -> OptionalValue<ManagedByteArray<HASH_LENGTH>> {
        let mut id_to_check = proposal_id;
        if self.proposal_root_hash(proposal_id).is_empty() {
            // check if proposal does not exist
            if proposal_id != 0 {
                // check inexistent proposal is not default
                id_to_check = 0;

                if self.proposal_root_hash(id_to_check).is_empty() {
                    // check default proposal is set
                    return OptionalValue::None;
                }
            } else {
                return OptionalValue::None;
            }
        }

        OptionalValue::Some(self.proposal_root_hash(id_to_check).get())
    }

    #[view(confirmVotingPower)]
    fn confirm_voting_power(
        &self,
        proposal_id: ProposalId,
        voting_power: BigUint<Self::Api>,
        proof: ManagedVec<ManagedByteArray<HASH_LENGTH>>,
    ) -> bool {
        match self.get_root_hash(proposal_id) {
            OptionalValue::None => {
                sc_panic!(NO_PROPOSAL);
            }
            OptionalValue::Some(root_hash) => {
                let caller = self.blockchain().get_caller();
                self.verify_merkle_proof(&caller, &voting_power, proof, root_hash)
            }
        }
    }

    fn verify_merkle_proof(
        &self,
        caller: &ManagedAddress,
        power: &BigUint<Self::Api>,
        proof: ManagedVec<ManagedByteArray<HASH_LENGTH>>,
        root_hash: ManagedByteArray<HASH_LENGTH>,
    ) -> bool {
        let mut leaf_bytes = caller.as_managed_buffer().clone();

        let p = power.to_bytes_be_buffer();
        leaf_bytes.append(&p);

        let mut hash = self.crypto().sha256(&leaf_bytes);
        for proof_item in proof {
            if BigUint::from(hash.as_managed_buffer())
                < BigUint::from(proof_item.as_managed_buffer())
            {
                let mut tst = hash.as_managed_buffer().clone();
                tst.append(proof_item.as_managed_buffer());

                hash = self.crypto().sha256(tst);
            } else {
                let mut tst = proof_item.as_managed_buffer().clone();
                tst.append(hash.as_managed_buffer());

                hash = self.crypto().sha256(tst);
            }
        }

        hash == root_hash
    }

    #[storage_mapper("proposalRootHash")]
    fn proposal_root_hash(&self, proposal_id: ProposalId) -> SingleValueMapper<Hash<Self::Api>>;
}
