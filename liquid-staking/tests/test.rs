mod contract_interactions;
mod contract_setup;
use std::ops::Mul;

use contract_setup::*;

use multiversx_sc_scenario::{num_bigint, rust_biguint, DebugApi};

#[test]
fn init_test() {
    LiquidStakingContractSetup::new(liquid_staking::contract_obj);
}

#[test]
fn liquid_staking_add_liquidity_test() {
    DebugApi::dummy();
    let mut sc_setup = LiquidStakingContractSetup::new(liquid_staking::contract_obj);

    sc_setup.deploy_staking_contract(&sc_setup.owner_address.clone(), 1000, 1000, 1500, 0, 0);

    let first_user = sc_setup.setup_new_user(100u64);
    sc_setup.add_liquidity(&first_user, 100u64);
    sc_setup.check_contract_storage(100, 100, 0, 0);
    sc_setup.check_user_balance(&first_user, LS_TOKEN_ID, 100u64);
}

#[test]
fn liquid_staking_remove_liquidity_test() {
    DebugApi::dummy();
    let mut sc_setup = LiquidStakingContractSetup::new(liquid_staking::contract_obj);

    sc_setup.deploy_staking_contract(&sc_setup.owner_address.clone(), 1000, 1000, 1500, 0, 0);

    let first_user = sc_setup.setup_new_user(100u64);
    sc_setup.add_liquidity(&first_user, 100u64);
    sc_setup.remove_liquidity(&first_user, LS_TOKEN_ID, 90u64);
    sc_setup.check_contract_storage(10, 10, 0, 0);
    sc_setup.check_user_balance(&first_user, LS_TOKEN_ID, 10u64);
    sc_setup.check_user_nft_balance_denominated(&first_user, UNSTAKE_TOKEN_ID, 1, 1);
    sc_setup.check_user_egld_balance(&first_user, 0u64);
}

#[test]
fn liquid_staking_claim_rewards_and_withdraw_test() {
    DebugApi::dummy();
    let mut sc_setup = LiquidStakingContractSetup::new(liquid_staking::contract_obj);

    let delegation_contract =
        sc_setup.deploy_staking_contract(&sc_setup.owner_address.clone(), 1000, 1000, 1500, 0, 0);

    let first_user = sc_setup.setup_new_user(100u64);
    sc_setup.add_liquidity(&first_user, 100u64);
    sc_setup.b_mock.set_block_epoch(50u64);
    sc_setup.b_mock.set_block_nonce(10u64);
    sc_setup.claim_rewards(&first_user);
    sc_setup.b_mock.set_block_nonce(20u64);
    sc_setup.recompute_token_reserve(&first_user);
    sc_setup.delegate_rewards(&first_user);

    sc_setup.remove_liquidity(&first_user, LS_TOKEN_ID, 90u64);

    sc_setup.b_mock.set_block_epoch(60u64);
    sc_setup.withdraw_all(&first_user, &delegation_contract);
    sc_setup.unbond_tokens(&first_user, UNSTAKE_TOKEN_ID, 1);

    sc_setup.check_user_balance(&first_user, LS_TOKEN_ID, 10u64);
    sc_setup.check_user_egld_balance_denominated(&first_user, 91232876712328767122u128);
}

#[test]
fn liquid_staking_multiple_operations() {
    DebugApi::dummy();
    let mut sc_setup = LiquidStakingContractSetup::new(liquid_staking::contract_obj);

    let delegation_contract1 = sc_setup.deploy_staking_contract(
        &sc_setup.owner_address.clone(),
        1000,
        1000,
        1100,
        3,
        10_000u64,
    );
    let delegation_contract2 = sc_setup.deploy_staking_contract(
        &sc_setup.owner_address.clone(),
        1000,
        1000,
        1100,
        3,
        13_000u64,
    );
    let delegation_contract3 = sc_setup.deploy_staking_contract(
        &sc_setup.owner_address.clone(),
        1000,
        1000,
        1100,
        3,
        11_000u64,
    );

    let first_user = sc_setup.setup_new_user(100u64);
    let second_user = sc_setup.setup_new_user(100u64);
    let third_user = sc_setup.setup_new_user(100u64);
    sc_setup.add_liquidity(&first_user, 10u64);
    sc_setup.check_delegation_contract_values(&delegation_contract2, 10u64);

    sc_setup.add_liquidity(&first_user, 20u64);
    sc_setup.check_delegation_contract_values(&delegation_contract2, 30u64);

    sc_setup.add_liquidity(&second_user, 50u64);
    sc_setup.check_delegation_contract_values(&delegation_contract2, 80u64);

    sc_setup.update_staking_contract_params(
        &sc_setup.owner_address.clone(),
        &delegation_contract2,
        1080,
        1100,
        3,
        13_000u64,
    );

    sc_setup.add_liquidity(&third_user, 30u64);
    sc_setup.check_delegation_contract_values(&delegation_contract3, 30u64);

    sc_setup.update_staking_contract_params(
        &sc_setup.owner_address.clone(),
        &delegation_contract2,
        1080,
        1100,
        3,
        8_000u64,
    );
    sc_setup.update_staking_contract_params(
        &sc_setup.owner_address.clone(),
        &delegation_contract3,
        1030,
        1100,
        3,
        9_000u64,
    );

    sc_setup.check_user_balance(&first_user, LS_TOKEN_ID, 30u64);
    sc_setup.check_user_balance(&second_user, LS_TOKEN_ID, 50u64);
    sc_setup.check_user_balance(&third_user, LS_TOKEN_ID, 30u64);

    sc_setup.b_mock.set_block_epoch(10u64);
    sc_setup.claim_rewards(&first_user);
    sc_setup.b_mock.set_block_nonce(10u64);
    sc_setup.recompute_token_reserve(&first_user);
    sc_setup.check_user_egld_balance_denominated(
        sc_setup.sc_wrapper.address_ref(),
        301369863013698629u128,
    );
    sc_setup.check_contract_rewards_storage_denominated(301369863013698629u128);
    sc_setup.delegate_rewards_check_insufficient(&first_user);

    sc_setup.add_liquidity(&third_user, 10u64);

    sc_setup.b_mock.set_block_epoch(50u64);
    sc_setup.claim_rewards(&first_user);
    sc_setup.b_mock.set_block_nonce(20u64);
    sc_setup.recompute_token_reserve(&first_user);
    sc_setup.check_user_egld_balance_denominated(
        sc_setup.sc_wrapper.address_ref(),
        1643835616438356161u128,
    );
    sc_setup.check_contract_rewards_storage_denominated(1643835616438356161u128);
    sc_setup.delegate_rewards(&first_user);
    sc_setup.check_user_egld_balance_denominated(sc_setup.sc_wrapper.address_ref(), 0u128);

    sc_setup.add_liquidity(&first_user, 50u64);
    sc_setup.check_delegation_contract_values_denominated(
        &delegation_contract1,
        61643835616438356161u128,
    );
    sc_setup.update_staking_contract_params(
        &sc_setup.owner_address.clone(),
        &delegation_contract1,
        1061,
        1100,
        3,
        10_000u64,
    );

    sc_setup.add_liquidity(&second_user, 40u64);
    sc_setup.check_delegation_contract_values_denominated(
        &delegation_contract1,
        61643835616438356161u128,
    );
    sc_setup.update_staking_contract_params(
        &sc_setup.owner_address.clone(),
        &delegation_contract1,
        1090,
        1100,
        3,
        10_000u64,
    );

    sc_setup.add_liquidity(&third_user, 30u64);
    sc_setup.check_delegation_contract_values(&delegation_contract3, 100u64);
    sc_setup.update_staking_contract_params(
        &sc_setup.owner_address.clone(),
        &delegation_contract3,
        1100,
        1100,
        3,
        9_000u64,
    );

    sc_setup.check_user_balance_denominated(&first_user, LS_TOKEN_ID, 79324324324324324325u128);
    sc_setup.check_user_balance_denominated(&second_user, LS_TOKEN_ID, 89459459459459459460u128);
    sc_setup.check_user_balance_denominated(&third_user, LS_TOKEN_ID, 69594594594594594595u128);

    sc_setup.remove_liquidity(&first_user, LS_TOKEN_ID, 70u64);
    sc_setup.check_user_balance_denominated(&first_user, LS_TOKEN_ID, 9324324324324324325u128);

    sc_setup.b_mock.set_block_epoch(60u64);
    sc_setup.check_user_egld_balance(&first_user, 20u64);
    sc_setup.withdraw_all(&first_user, &delegation_contract1);
    sc_setup.withdraw_all(&first_user, &delegation_contract2);
    sc_setup.withdraw_all(&first_user, &delegation_contract3);
    sc_setup.unbond_tokens(&first_user, UNSTAKE_TOKEN_ID, 1);

    let ls_value = sc_setup.get_ls_value_for_position(1u64);
    let initial_egld_balance = exp18_128(20u64);
    let ls_token_balance_in_egld = 70 * ls_value;
    let rounding_offset = 24u128;
    sc_setup.check_user_egld_balance_denominated(
        &first_user,
        initial_egld_balance + ls_token_balance_in_egld + rounding_offset,
    );
}

#[test]
fn liquid_staking_multiple_withdraw_test() {
    DebugApi::dummy();
    let mut sc_setup = LiquidStakingContractSetup::new(liquid_staking::contract_obj);

    let delegation_contract =
        sc_setup.deploy_staking_contract(&sc_setup.owner_address.clone(), 1000, 1000, 1500, 0, 0);

    let first_user = sc_setup.setup_new_user(100u64);
    let second_user = sc_setup.setup_new_user(100u64);
    let third_user = sc_setup.setup_new_user(100u64);
    sc_setup.add_liquidity(&first_user, 50u64);
    sc_setup.add_liquidity(&second_user, 40u64);
    sc_setup.add_liquidity(&third_user, 40u64);
    sc_setup.check_contract_storage(130, 130, 0, 0);

    sc_setup.b_mock.set_block_epoch(50u64);
    sc_setup.remove_liquidity(&first_user, LS_TOKEN_ID, 20u64);
    sc_setup.remove_liquidity(&second_user, LS_TOKEN_ID, 20u64);
    sc_setup.remove_liquidity(&third_user, LS_TOKEN_ID, 20u64);
    sc_setup.check_contract_storage(70, 70, 0, 0);

    sc_setup.b_mock.set_block_epoch(60u64);
    sc_setup.withdraw_all(&first_user, &delegation_contract);
    sc_setup.unbond_tokens(&first_user, UNSTAKE_TOKEN_ID, 1);
    sc_setup.check_user_balance(&first_user, LS_TOKEN_ID, 30u64);
    sc_setup.check_user_egld_balance(&first_user, 70);
    sc_setup.check_user_balance(&second_user, LS_TOKEN_ID, 20u64);
    sc_setup.check_user_egld_balance(&second_user, 60);
    sc_setup.check_user_balance(&third_user, LS_TOKEN_ID, 20u64);
    sc_setup.check_user_egld_balance(&third_user, 60);
    sc_setup.check_contract_storage(70, 70, 0, 40); // 20 + 20 (second_user + third_user)
}

pub fn exp9(value: u64) -> num_bigint::BigUint {
    value.mul(rust_biguint!(10).pow(9))
}

pub fn exp15(value: u64) -> num_bigint::BigUint {
    value.mul(rust_biguint!(10).pow(15))
}

pub fn exp17(value: u64) -> num_bigint::BigUint {
    value.mul(rust_biguint!(10).pow(17))
}

pub fn exp18(value: u64) -> num_bigint::BigUint {
    value.mul(rust_biguint!(10).pow(18))
}

pub fn exp18_128(value: u64) -> u128 {
    u128::from(value).mul(10u128.pow(18))
}
