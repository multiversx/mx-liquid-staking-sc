multiversx_sc::imports!();

use crate::{basics::errors::ERROR_INSUFFICIENT_LIQ_BURNED, liquidity_pool, setup};

#[multiversx_sc::module]
pub trait ViewsModule:
    setup::config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + liquidity_pool::LiquidityPoolModule
    + setup::governance::GovernanceModule
{
    // views
    #[view(getLsValueForPosition)]
    fn get_ls_value_for_position(&self, ls_token_amount: BigUint) -> BigUint {
        let ls_token_supply = self.ls_token_supply().get();
        let virtual_egld_reserve = self.virtual_egld_reserve().get();

        let egld_amount = ls_token_amount * &virtual_egld_reserve / ls_token_supply;
        require!(egld_amount > 0u64, ERROR_INSUFFICIENT_LIQ_BURNED);

        egld_amount
    }
}
