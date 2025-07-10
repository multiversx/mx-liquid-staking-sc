use crate::basics::errors::{ERROR_GOVERNANCE_SC_NOT_SET, ERROR_INVALID_SC_ADDRESS};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait VoteModule {
    #[only_owner]
    #[endpoint]
    fn set_governance_contract(&self, sc_address: ManagedAddress) {
        self.governance_contract().set(sc_address);
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

    #[storage_mapper("voteContract")]
    fn governance_contract(&self) -> SingleValueMapper<ManagedAddress>;
}
