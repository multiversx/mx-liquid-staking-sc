elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::liquidity_pool::State;

pub const MAX_PERCENTAGE: u64 = 100_000;
pub const UNBOND_PERIOD: u64 = 10;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Debug)]
pub struct UnstakeTokenAttributes<M: ManagedTypeApi> {
    pub delegation_contract: ManagedAddress<M>,
    pub unstake_epoch: u64,
    pub unbond_epoch: u64,
}

#[elrond_wasm::module]
pub trait ConfigModule {
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(registerLsToken)]
    fn register_ls_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        let payment_amount = self.call_value().egld_value();
        self.ls_token().issue_and_set_all_roles(
            payment_amount,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(registerUnstakeToken)]
    fn register_unstake_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        let payment_amount = self.call_value().egld_value();
        self.unstake_token().issue_and_set_all_roles(
            EsdtTokenType::Meta,
            payment_amount,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    #[only_owner]
    #[endpoint(setStateActive)]
    fn set_state_active(&self) {
        self.state().set(State::Active);
    }

    #[only_owner]
    #[endpoint(setStateInactive)]
    fn set_state_inactive(&self) {
        self.state().set(State::Inactive);
    }

    #[inline]
    fn is_state_active(&self, state: State) -> bool {
        state == State::Active
    }

    #[view(getState)]
    #[storage_mapper("state")]
    fn state(&self) -> SingleValueMapper<State>;

    #[view(getLsTokenId)]
    #[storage_mapper("ls_token_id")]
    fn ls_token(&self) -> FungibleTokenMapper<Self::Api>;

    #[view(getLsSupply)]
    #[storage_mapper("ls_token_supply")]
    fn ls_token_supply(&self) -> SingleValueMapper<BigUint>;

    #[view(getVirtualEGLDReserve)]
    #[storage_mapper("virtual_egld_reserve")]
    fn virtual_egld_reserve(&self) -> SingleValueMapper<BigUint>;

    #[view(getRewardsReserve)]
    #[storage_mapper("rewards_reserve")]
    fn rewards_reserve(&self) -> SingleValueMapper<BigUint>;

    #[view(getWithdrawnEGLD)]
    #[storage_mapper("withdrawn_egld")]
    fn withdrawn_egld(&self) -> SingleValueMapper<BigUint>;

    #[view(getUnstakeTokenId)]
    #[storage_mapper("unstake_token_id")]
    fn unstake_token(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[view(getUnstakeTokenSupply)]
    #[storage_mapper("unstake_token_supply")]
    fn unstake_token_supply(&self) -> SingleValueMapper<BigUint>;
}
