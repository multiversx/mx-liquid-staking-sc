elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::State;

pub struct StorageCache<'a, C>
where
    C: crate::config::ConfigModule,
{
    sc_ref: &'a C,
    pub contract_state: State,
    pub ls_token_id: TokenIdentifier<C::Api>,
    pub ls_token_supply: BigUint<C::Api>,
    pub virtual_egld_reserve: BigUint<C::Api>,
    pub rewards_reserve: BigUint<C::Api>,
}

impl<'a, C> StorageCache<'a, C>
where
    C: crate::config::ConfigModule,
{
    pub fn new(sc_ref: &'a C) -> Self {

        StorageCache {
            contract_state: sc_ref.state().get(),
            ls_token_id: sc_ref.ls_token().get_token_id(),
            ls_token_supply: sc_ref.ls_token_supply().get(),
            virtual_egld_reserve: sc_ref.virtual_egld_reserve().get(),
            rewards_reserve: sc_ref.rewards_reserve().get(),
            sc_ref,
        }
    }
}

impl<'a, C> Drop for StorageCache<'a, C>
where
    C: crate::config::ConfigModule,
{
    fn drop(&mut self) {
        // commit changes to storage for the mutable fields
        self.sc_ref.ls_token_supply().set(&self.ls_token_supply);
        self.sc_ref.virtual_egld_reserve().set(&self.virtual_egld_reserve);
        self.sc_ref.rewards_reserve().set(&self.rewards_reserve);
    }
}
