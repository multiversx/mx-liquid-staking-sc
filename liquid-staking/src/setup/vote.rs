use crate::basics::errors::ERROR_VOTE_SC_NOT_SET;

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
        self.vote_contract().get()
    }

    #[storage_mapper("voteContract")]
    fn vote_contract(&self) -> SingleValueMapper<ManagedAddress>;
}
