#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod basics;
pub mod contexts;
pub mod funds;
pub mod liquidity;
pub mod liquidity_pool;
pub mod setup;

use setup::delegation::{ClaimStatus, ClaimStatusType};

use contexts::base::*;
use liquidity_pool::State;

#[multiversx_sc::contract]
pub trait LiquidStaking:
    basics::events::EventsModule
    + basics::views::ViewsModule
    + setup::config::ConfigModule
    + setup::delegation::DelegationModule
    + setup::vote::VoteModule
    + funds::claim::ClaimModule
    + funds::delegate_rewards::DelegateRewardsModule
    + funds::recompute_token_reserve::RecomputeTokenReserveModule
    + funds::unbond::UnbondModule
    + funds::withdraw::WithdrawModule
    + liquidity::add_liquidity::AddLiquidityModule
    + liquidity::remove_liquidity::RemoveLiquidityModule
    + liquidity_pool::LiquidityPoolModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[init]
    fn init(&self) {
        self.state().set(State::Inactive);
        let current_epoch = self.blockchain().get_block_epoch();
        let current_round = self.blockchain().get_block_round();
        let claim_status = ClaimStatus {
            status: ClaimStatusType::Insufficient,
            last_claim_epoch: current_epoch,
            last_claim_block: current_round,
        };
        self.delegation_claim_status().set_if_empty(claim_status);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
