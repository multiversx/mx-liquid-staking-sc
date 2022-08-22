mod contract_interactions;
mod contract_setup;
use std::ops::Mul;

use contract_setup::*;

use elrond_wasm_debug::{num_bigint, rust_biguint, DebugApi};

#[test]
fn init_test() {
    let _ = LiquidStakingContractSetup::new(liquid_staking::contract_obj);
}

#[test]
fn liquid_staking_add_liquidity_test() {
    let _ = DebugApi::dummy();
    let mut sc_setup = LiquidStakingContractSetup::new(liquid_staking::contract_obj);

    sc_setup.deploy_staking_contract(&sc_setup.owner_address.clone(), 1000, 1000, 0, 0, 0);

    let first_user = sc_setup.setup_new_user(100u64);
    sc_setup.add_liquidity(&first_user, 100u64);
    sc_setup.check_contract_storage(100, 100, 0, 0, 0);
    sc_setup.check_user_balance(&first_user, LS_TOKEN_ID, 100u64);
}

#[test]
fn liquid_staking_remove_liquidity_test() {
    let _ = DebugApi::dummy();
    let mut sc_setup = LiquidStakingContractSetup::new(liquid_staking::contract_obj);

    sc_setup.deploy_staking_contract(&sc_setup.owner_address.clone(), 1000, 1000, 0, 0, 0);

    let first_user = sc_setup.setup_new_user(100u64);
    sc_setup.add_liquidity(&first_user, 100u64);
    sc_setup.remove_liquidity(&first_user, LS_TOKEN_ID, 90u64);
    sc_setup.check_contract_storage(10, 100, 0, 0, 90);
    sc_setup.check_user_balance(&first_user, LS_TOKEN_ID, 10u64);
    sc_setup.check_user_nft_balance(&first_user, UNSTAKE_TOKEN_ID, 1, 90u64);
    sc_setup.check_user_egld_balance(&first_user, 0u64);
}

#[test]
fn liquid_staking_claim_rewards_and_withdraw_test() {
    let _ = DebugApi::dummy();
    let mut sc_setup = LiquidStakingContractSetup::new(liquid_staking::contract_obj);

    sc_setup.deploy_staking_contract(&sc_setup.owner_address.clone(), 1000, 1000, 0, 0, 0);

    let first_user = sc_setup.setup_new_user(100u64);
    sc_setup.add_liquidity(&first_user, 100u64);
    sc_setup.b_mock.set_block_epoch(5u64);
    sc_setup.claim_rewards(&first_user);
    sc_setup.delegate_rewards(&first_user);

    sc_setup.remove_liquidity(&first_user, LS_TOKEN_ID, 90u64);

    sc_setup.b_mock.set_block_epoch(15u64);
    sc_setup.unbond_tokens(&first_user, UNSTAKE_TOKEN_ID, 1, 90u64);

    sc_setup.check_user_balance(&first_user, LS_TOKEN_ID, 10u64);
    sc_setup.check_user_egld_balance(&first_user, 90u64);
    sc_setup.check_user_nft_balance_denominated(
        &first_user,
        UNSTAKE_TOKEN_ID,
        1,
        123287671232876711,
    );
}

pub fn exp15(value: u64) -> num_bigint::BigUint {
    value.mul(rust_biguint!(10).pow(15))
}

pub fn exp17(value: u64) -> num_bigint::BigUint {
    value.mul(rust_biguint!(10).pow(17))
}

pub fn exp9(value: u64) -> num_bigint::BigUint {
    value.mul(rust_biguint!(10).pow(9))
}
