use crate::basics::{
    constants::{Epoch, Timestamp},
    errors::{ERROR_GOVERNANCE_SC_NOT_SET, ERROR_INVALID_SC_ADDRESS},
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Eq, Debug, ManagedVecItem,
)]
pub struct LockedFunds<M>
where
    M: ManagedTypeApi,
{
    pub funds: EsdtTokenPayment<M>,
    pub claim_back: Timestamp,
}

#[multiversx_sc::module]
pub trait VoteModule {
    #[only_owner]
    #[endpoint]
    fn set_governance_contract(&self, sc_address: ManagedAddress) {
        self.governance_contract().set(sc_address);
    }

    #[only_owner]
    #[endpoint]
    fn set_lock_vote_period(&self, sc_address: ManagedAddress) {
        self.governance_contract().set(sc_address);
    }

    #[only_owner]
    #[endpoint]
    fn set_porposal_end_period(&self, proposal: ManagedBuffer, end_period: Epoch) {
        self.proposal_end_period(proposal).set(end_period);
    }

    fn get_governance_sc(&self) -> ManagedAddress {
        require!(
            !self.governance_contract().is_empty(),
            ERROR_GOVERNANCE_SC_NOT_SET
        );
        let address = self.governance_contract().get();
        self.require_sc_address(&address);
        address
    }

    fn require_sc_address(&self, address: &ManagedAddress) {
        require!(
            !address.is_zero() && self.blockchain().is_smart_contract(address),
            ERROR_INVALID_SC_ADDRESS
        );
    }

    #[storage_mapper("governanceContract")]
    fn governance_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("proposalEndPeriod")]
    fn proposal_end_period(&self, proposal: ManagedBuffer) -> SingleValueMapper<Epoch>;

    #[storage_mapper("voteLockPeriod")]
    fn vote_lock_period(&self) -> SingleValueMapper<Timestamp>;

    #[storage_mapper("lockedVoteBalanceAndProposals")]
    fn locked_vote_balance(
        &self,
        address: &ManagedAddress,
    ) -> SingleValueMapper<LockedFunds<Self::Api>>;

    #[storage_mapper("votedProposals")]
    fn voted_proposals(&self, address: &ManagedAddress) -> UnorderedSetMapper<ManagedBuffer>;
}
