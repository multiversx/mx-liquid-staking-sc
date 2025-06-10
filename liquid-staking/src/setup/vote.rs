use crate::basics::errors::{ERROR_INVALID_SC_ADDRESS, ERROR_VOTE_SC_NOT_SET};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait VoteModule {
    #[only_owner]
    #[endpoint]
    fn set_vote_contract(&self, sc_address: ManagedAddress) {
        self.vote_contract().set(sc_address);
    }

    fn get_vote_sc(&self) -> ManagedAddress {
        require!(!self.vote_contract().is_empty(), ERROR_VOTE_SC_NOT_SET);
        let address = self.vote_contract().get();
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
    fn vote_contract(&self) -> SingleValueMapper<ManagedAddress>;
}
