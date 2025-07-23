multiversx_sc::imports!();

use crate::{
    constants::{Hash, ProposalId, HASH_LENGTH, PROOF_LENGTH},
    errors::NO_PROPOSAL,
};

#[multiversx_sc::module]
pub trait ViewsModule {
    #[view(getProposalRootHash)]
    fn get_root_hash(
        &self,
        proposal_id: ProposalId,
    ) -> OptionalValue<ManagedByteArray<HASH_LENGTH>> {
        if self.root_hash_proposal_nonce(proposal_id).is_empty() {
            return OptionalValue::None;
        }

        OptionalValue::Some(self.root_hash_proposal_nonce(proposal_id).get())
    }

    #[view(confirmVotingPower)]
    fn confirm_voting_power(
        &self,
        proposal_id: ProposalId,
        voting_power: BigUint<Self::Api>,
        proof: ArrayVec<ManagedByteArray<HASH_LENGTH>, PROOF_LENGTH>,
    ) -> bool {
        match self.get_root_hash(proposal_id) {
            OptionalValue::None => {
                sc_panic!(NO_PROPOSAL);
            }
            OptionalValue::Some(root_hash) => {
                self.verify_merkle_proof(voting_power, proof, root_hash)
            }
        }
    }

    fn verify_merkle_proof(
        &self,
        power: BigUint<Self::Api>,
        proof: ArrayVec<ManagedByteArray<HASH_LENGTH>, PROOF_LENGTH>,
        root_hash: ManagedByteArray<HASH_LENGTH>,
    ) -> bool {
        let caller = self.blockchain().get_caller();
        let mut leaf_bytes = caller.as_managed_buffer().clone();

        let p = power.to_bytes_be_buffer();
        leaf_bytes.append(&p);

        let mut hash = self.crypto().sha256(&leaf_bytes);
        for proof_item in proof {
            if hash.as_managed_buffer() < proof_item.as_managed_buffer() {
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

    #[storage_mapper("rootHashProposalNonce")]
    fn root_hash_proposal_nonce(
        &self,
        proposal_id: ProposalId,
    ) -> SingleValueMapper<Hash<Self::Api>>;
}
