use crate::errors::SELF_CALL_ERROR;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait CallerCheckModule {
    fn require_caller_not_self(&self, caller: &ManagedAddress) {
        let sc_address = self.blockchain().get_sc_address();

        require!(caller != &sc_address, SELF_CALL_ERROR);
    }
}
