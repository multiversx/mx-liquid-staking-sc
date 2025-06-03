multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::liquidity_pool::State;

pub const MAX_PERCENTAGE: u64 = 100_000;
pub const UNBOND_PERIOD: u64 = 10;

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Eq, Debug, ManagedVecItem,
)]
pub struct UnstakeTokenAttributes<M: ManagedTypeApi> {
    pub delegation_contract: ManagedAddress<M>,
    pub unstake_epoch: u64,
    pub unstake_amount: BigUint<M>,
    pub unbond_epoch: u64,
}

#[multiversx_sc::module]
pub trait ConfigModule:
    multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(registerLsToken)]
    fn register_ls_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        let payment_amount = self.call_value().egld().clone_value();
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
        let payment_amount = self.call_value().egld().clone_value();
        self.unstake_token().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            payment_amount,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    #[view(getState)]
    #[storage_mapper("state")]
    fn state(&self) -> SingleValueMapper<State>;

    #[view(getLsTokenId)]
    #[storage_mapper("lsTokenId")]
    fn ls_token(&self) -> FungibleTokenMapper<Self::Api>;

    #[view(getLsSupply)]
    #[storage_mapper("lsTokenSupply")]
    fn ls_token_supply(&self) -> SingleValueMapper<BigUint>;

    #[view(getVirtualEgldReserve)]
    #[storage_mapper("virtualEgldReserve")]
    fn virtual_egld_reserve(&self) -> SingleValueMapper<BigUint>;

    #[view(getRewardsReserve)]
    #[storage_mapper("rewardsReserve")]
    fn rewards_reserve(&self) -> SingleValueMapper<BigUint>;

    #[view(getUnstakeTokenId)]
    #[storage_mapper("unstakeTokenId")]
    fn unstake_token(&self) -> NonFungibleTokenMapper<Self::Api>;
}
