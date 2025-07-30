use crate::basics::{
    constants::Timestamp,
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
pub trait GovernanceModule:
    multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[only_owner]
    #[endpoint]
    fn set_governance_contract(&self, sc_address: ManagedAddress) {
        self.require_sc_address(&sc_address);
        self.governance_contract().set(sc_address);
    }

    fn get_governance_sc(&self) -> ManagedAddress {
        require!(
            !self.governance_contract().is_empty(),
            ERROR_GOVERNANCE_SC_NOT_SET
        );
        let address = self.governance_contract().get();

        address
    }

    fn require_sc_address(&self, address: &ManagedAddress) {
        require!(
            !address.is_zero() && self.blockchain().is_smart_contract(address),
            ERROR_INVALID_SC_ADDRESS
        );
    }

    #[view]
    #[storage_mapper("governanceContract")]
    fn governance_contract(&self) -> SingleValueMapper<ManagedAddress>;
}
