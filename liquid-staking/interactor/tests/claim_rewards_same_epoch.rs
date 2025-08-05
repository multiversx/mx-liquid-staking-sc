use liquid_staking_interactor::Config;
use liquid_staking_interactor::LiquidStakingInteract;
use multiversx_sc_snippets::imports::*;

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_claim_rewards_same_epoch() {
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
    interact.generate_blocks_until_next_epoch().await;
    interact.claim_rewards(owner_address.clone(), None).await;
    interact.generate_blocks(8).await;
    interact
        .recompute_token_reserve(owner_address.clone())
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
