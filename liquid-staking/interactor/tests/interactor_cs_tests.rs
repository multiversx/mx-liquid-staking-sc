use liquid_staking_interactor::Config;
use liquid_staking_interactor::LiquidStakingInteract;
use multiversx_sc_snippets::imports::*;

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_unbond_tokens_happy_path() {
    let mut interact = LiquidStakingInteract::new(Config::chain_simulator_config()).await;
    let owner_address = Bech32Address::from(interact.wallet_address.clone());
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact
        .whitelist_delegation_contract(
            1_000_000_000_000_000_000u128,
            interact.state.delegation_address().clone(),
            owner_address.clone(),
            0u128,
            5_000_000_000_000_000_000u128,
            1u64,
            50_000u64,
        )
        .await;
    interact.set_state_active().await;
    let ls_token = interact
        .register_ls_token("LIQTEST", "LTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    let us_token = interact
        .register_unstake_token("UNSTAKETEST", "UNTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact.generate_blocks_until_epoch(20).await;
    interact
        .remove_liquidity(
            owner_address.clone(),
            &ls_token,
            1_000_000_000_000_000_001u128,
        )
        .await;
    interact.generate_blocks_until_epoch(30).await;
    interact.withdraw_all(owner_address.clone(), None).await;
    interact
        .unbond_tokens(owner_address.clone(), &us_token, 1u128)
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn remove_liquidity_and_withdraw_early() {
    let mut interact = LiquidStakingInteract::new(Config::chain_simulator_config()).await;
    let owner_address = Bech32Address::from(interact.wallet_address.clone());
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact
        .whitelist_delegation_contract(
            1_000_000_000_000_000_000u128,
            interact.state.delegation_address().clone(),
            owner_address.clone(),
            0u128,
            5_000_000_000_000_000_000u128,
            1u64,
            50_000u64,
        )
        .await;
    interact.set_state_active().await;
    let ls_token = interact
        .register_ls_token("LIQTEST", "LTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    let _ = interact
        .register_unstake_token("UNSTAKETEST", "UNTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .remove_liquidity(
            owner_address.clone(),
            &ls_token,
            1_000_000_000_000_000_001u128,
        )
        .await;
    interact
        .withdraw_all(owner_address, Some(ExpectError(4, "nothing to unBond")))
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_claim_rewards_happy_path() {
    let mut interact = LiquidStakingInteract::new(Config::chain_simulator_config()).await;
    let owner_address = Bech32Address::from(interact.wallet_address.clone());
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact
        .whitelist_delegation_contract(
            1_000_000_000_000_000_000u128,
            interact.state.delegation_address().clone(),
            owner_address.clone(),
            0u128,
            5_000_000_000_000_000_000u128,
            1u64,
            50_000u64,
        )
        .await;
    interact.set_state_active().await;
    let _ = interact
        .register_ls_token("LIQTEST", "LTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    let _ = interact
        .register_unstake_token("UNSTAKETEST", "UNTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact.generate_blocks_until_epoch(5).await;
    interact.claim_rewards(owner_address.clone(), None).await;
    interact.generate_blocks_until_epoch(10).await;
    interact.recompute_token_reserve(owner_address).await;
    interact.rewards_reserve().await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_claim_rewards_same_epoch() {
    // to fix
    let mut interact = LiquidStakingInteract::new(Config::chain_simulator_config()).await;
    let owner_address = Bech32Address::from(interact.wallet_address.clone());
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact
        .whitelist_delegation_contract(
            1_000_000_000_000_000_000u128,
            interact.state.delegation_address().clone(),
            owner_address.clone(),
            0u128,
            5_000_000_000_000_000_000u128,
            1u64,
            50_000u64,
        )
        .await;
    interact.set_state_active().await;
    let _ = interact
        .register_ls_token("LIQTEST", "LTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    let _ = interact
        .register_unstake_token("UNSTAKETEST", "UNTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .claim_rewards(
            owner_address,
            Some(ExpectError(
                4,
                "The rewards were already claimed for this epoch",
            )),
        )
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_claim_rewards_multiple_times_no_redelegation() {
    let mut interact = LiquidStakingInteract::new(Config::chain_simulator_config()).await;
    let owner_address = Bech32Address::from(interact.wallet_address.clone());
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact
        .whitelist_delegation_contract(
            1_000_000_000_000_000_000u128,
            interact.state.delegation_address().clone(),
            owner_address.clone(),
            0u128,
            5_000_000_000_000_000_000u128,
            1u64,
            50_000u64,
        )
        .await;
    interact.set_state_active().await;
    let _ = interact
        .register_ls_token("LIQTEST", "LTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    let _ = interact
        .register_unstake_token("UNSTAKETEST", "UNTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact.generate_blocks_until_epoch(5).await;
    interact.claim_rewards(owner_address.clone(), None).await;
    interact
        .claim_rewards(
            owner_address,
            Some(ExpectError(
                4,
                "Previous claimed rewards must be redelegated or lesser than 1 EGLD",
            )),
        )
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_multiple_claim_rewards() {
    let mut interact = LiquidStakingInteract::new(Config::chain_simulator_config()).await;
    let owner_address = Bech32Address::from(interact.wallet_address.clone());
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact
        .whitelist_delegation_contract(
            1_000_000_000_000_000_000u128,
            interact.state.delegation_address().clone(),
            owner_address.clone(),
            0u128,
            5_000_000_000_000_000_000u128,
            1u64,
            50_000u64,
        )
        .await;
    interact.set_state_active().await;
    let _ = interact
        .register_ls_token("LIQTEST", "LTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    let _ = interact
        .register_unstake_token("UNSTAKETEST", "UNTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact.generate_blocks_until_epoch(5).await;
    interact.claim_rewards(owner_address.clone(), None).await;
    interact.generate_blocks_until_epoch(10).await;
    interact
        .recompute_token_reserve(owner_address.clone())
        .await;
    interact.rewards_reserve().await;
    interact.generate_blocks_until_epoch(15).await;
    interact.claim_rewards(owner_address.clone(), None).await;
    interact.generate_blocks_until_epoch(20).await;
    interact.recompute_token_reserve(owner_address).await;
    interact.rewards_reserve().await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_delegate_not_enough_egld() {
    let mut interact = LiquidStakingInteract::new(Config::chain_simulator_config()).await;
    let owner_address = Bech32Address::from(interact.wallet_address.clone());
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact
        .whitelist_delegation_contract(
            1_000_000_000_000_000_000u128,
            interact.state.delegation_address().clone(),
            owner_address.clone(),
            0u128,
            5_000_000_000_000_000_000u128,
            1u64,
            50_000u64,
        )
        .await;
    interact.set_state_active().await;
    let _ = interact
        .register_ls_token("LIQTEST", "LTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    let _ = interact
        .register_unstake_token("UNSTAKETEST", "UNTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact.generate_blocks_until_epoch(5).await;
    interact.claim_rewards(owner_address.clone(), None).await;
    interact.generate_blocks_until_epoch(10).await;
    interact
        .recompute_token_reserve(owner_address.clone())
        .await;
    interact.rewards_reserve().await;
    interact
        .delegate_rewards(
            owner_address,
            Some(ExpectError(
                4,
                "Old claimed rewards must be greater than 1 EGLD",
            )),
        )
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn delegate_vote() {
    let mut interact = LiquidStakingInteract::new(Config::chain_simulator_config()).await;
    let owner_address = Bech32Address::from(interact.wallet_address.clone());
    interact.deploy().await;
    interact.deploy_governance_contract().await;
    interact.deploy_delegation_contract().await;
    interact
        .whitelist_delegation_contract(
            1_000_000_000_000_000_000u128,
            interact.state.delegation_address().clone(),
            owner_address.clone(),
            0u128,
            5_000_000_000_000_000_000u128,
            1u64,
            50_000u64,
        )
        .await;
    interact.set_state_active().await;
    let _ = interact
        .register_ls_token("LIQTEST", "LTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    let _ = interact
        .register_unstake_token("UNSTAKETEST", "UNTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address.clone(), 1_000_000_000_000_000_001u128)
        .await;
    interact
        .add_liquidity(owner_address, 1_000_000_000_000_000_001u128)
        .await;
    interact.deploy_vote_contract().await;
    interact.delegate_vote(None).await;
}
