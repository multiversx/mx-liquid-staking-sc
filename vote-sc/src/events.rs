use crate::constants::ProposalId;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("VoteCast")]
    fn vote_cast_event(
        &self,
        #[indexed] voter: &ManagedAddress,
        #[indexed] proposal_id: ProposalId,
        #[indexed] voting_power: &BigUint,
        #[indexed] user_quorum: &BigUint,
    );
}
