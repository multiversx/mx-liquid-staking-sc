elrond_wasm::imports!();
elrond_wasm::derive_imports!();
use crate::elrond_codec::TopEncode;

use crate::contexts::base::StorageCache;
use crate::errors::*;

use super::config;

const MINIMUM_LIQUIDITY: u64 = 1_000;

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Eq, Copy, Clone, Debug)]
pub enum State {
    Inactive,
    Active,
}

#[elrond_wasm::module]
pub trait LiquidityPoolModule:
    config::ConfigModule + elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    fn pool_add_liquidity(
        &self,
        token_amount: &BigUint,
        storage_cache: &mut StorageCache<Self>,
    ) -> BigUint {
        let ls_amount = if storage_cache.virtual_egld_reserve > 0 {
            token_amount.clone() * &storage_cache.ls_token_supply
                / (&storage_cache.virtual_egld_reserve + &storage_cache.rewards_reserve)
        } else {
            token_amount.clone()
        };

        require!(ls_amount > 0, ERROR_INSUFFICIENT_LIQUIDITY);

        storage_cache.ls_token_supply += &ls_amount;
        storage_cache.virtual_egld_reserve += token_amount;

        ls_amount
    }

    fn pool_remove_liquidity(
        &self,
        token_amount: &BigUint,
        storage_cache: &mut StorageCache<Self>,
    ) -> BigUint {
        let egld_amount = self.get_egld_amount(token_amount.clone(), storage_cache);
        storage_cache.ls_token_supply -= token_amount;

        egld_amount
    }

    fn get_egld_amount(
        &self,
        ls_token_amount: BigUint,
        storage_cache: &StorageCache<Self>,
    ) -> BigUint {
        require!(
            storage_cache.ls_token_supply >= &ls_token_amount + MINIMUM_LIQUIDITY,
            ERROR_NOT_ENOUGH_LP
        );

        let egld_amount = (ls_token_amount
            * (&storage_cache.virtual_egld_reserve + &storage_cache.rewards_reserve))
            / &storage_cache.ls_token_supply;
        require!(egld_amount > 0u64, ERROR_INSUFFICIENT_LIQ_BURNED);

        egld_amount
    }

    fn mint_ls_token(&self, amount: BigUint) -> EsdtTokenPayment<Self::Api> {
        self.ls_token().mint(amount)
    }

    fn burn_ls_token(&self, amount: &BigUint) {
        self.ls_token().burn(amount);
    }

    fn mint_unstake_tokens<T: TopEncode>(&self, attributes: &T) -> EsdtTokenPayment<Self::Api> {
        self.unstake_token()
            .nft_create(BigUint::from(1u64), attributes)
    }

    fn burn_unstake_tokens(&self, token_nonce: u64) {
        self.unstake_token()
            .nft_burn(token_nonce, &BigUint::from(1u64));
    }
}
