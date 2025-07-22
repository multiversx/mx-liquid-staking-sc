multiversx_sc::imports!();

use crate::{basics::errors::ERROR_INSUFFICIENT_LIQ_BURNED, liquidity_pool, setup};

#[multiversx_sc::module]
pub trait ViewsModule:
    setup::config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + liquidity_pool::LiquidityPoolModule
    + setup::vote::VoteModule
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

    #[view(getVotingPower)]
    fn get_voting_power(&self, payment: EsdtTokenPayment) -> BigUint {
        let caller = self.blockchain().get_caller();
        let ls_token_id = self.ls_token().get_token_id();
        let ls_token_supply = self.ls_token_supply().get();
        let virtual_egld_reserve = self.virtual_egld_reserve().get();

        let mut available_ls_token = EsdtTokenPayment::new(ls_token_id.clone(), 0, BigUint::zero());

        if !self.locked_vote_balance(&caller).is_empty() {
            let locked_balance = self.locked_vote_balance(&caller).get();
            available_ls_token = locked_balance.funds;
        }
        self.get_egld_amount(
            &(available_ls_token.amount + payment.amount),
            &ls_token_supply,
            &virtual_egld_reserve,
        )
    }
}
