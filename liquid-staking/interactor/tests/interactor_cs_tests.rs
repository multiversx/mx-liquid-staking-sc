use liquid_staking_interactor::Config;
use liquid_staking_interactor::ContractInteract;
use multiversx_sc_snippets::imports::*;

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_unbond_tokens_happy_path() {
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact.whitelist_delegation_contract().await;
    interact.set_state_active().await;
    let ls_token = interact.register_ls_token().await;
    let us_token = interact.register_unstake_token().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.generate_blocks_until_epoch(20).await;
    interact.remove_liquidity(&ls_token).await;
    interact.generate_blocks_until_epoch(30).await;
    interact.withdraw_all(None).await;
    interact.unbond_tokens(&us_token).await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn remove_liquidity_and_withdraw_early() {
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact.whitelist_delegation_contract().await;
    interact.set_state_active().await;
    let ls_token = interact.register_ls_token().await;
    let _ = interact.register_unstake_token().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.remove_liquidity(&ls_token).await;
    interact
        .withdraw_all(Some(ExpectError(4, "Cannot withdraw yet")))
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_claim_rewards_happy_path() {
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact.whitelist_delegation_contract().await;
    interact.set_state_active().await;
    let _ = interact.register_ls_token().await;
    let _ = interact.register_unstake_token().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.generate_blocks_until_epoch(5).await;
    interact.claim_rewards(None).await;
    interact.generate_blocks_until_epoch(10).await;
    interact.recompute_token_reserve().await;
    interact.rewards_reserve().await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_claim_rewards_same_epoch() {
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact.whitelist_delegation_contract().await;
    interact.set_state_active().await;
    let _ = interact.register_ls_token().await;
    let _ = interact.register_unstake_token().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact
        .claim_rewards(Some(ExpectError(
            4,
            "The rewards were already claimed for this epoch",
        )))
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_claim_rewards_multiple_times_no_redelegation() {
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact.whitelist_delegation_contract().await;
    interact.set_state_active().await;
    let _ = interact.register_ls_token().await;
    let _ = interact.register_unstake_token().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.generate_blocks_until_epoch(5).await;
    interact.claim_rewards(None).await;
    interact
        .claim_rewards(Some(ExpectError(
            4,
            "Previous claimed rewards must be redelegated or lesser than 1 EGLD",
        )))
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_multiple_claim_rewards() {
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact.whitelist_delegation_contract().await;
    interact.set_state_active().await;
    let _ = interact.register_ls_token().await;
    let _ = interact.register_unstake_token().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.generate_blocks_until_epoch(5).await;
    interact.claim_rewards(None).await;
    interact.generate_blocks_until_epoch(10).await;
    interact.recompute_token_reserve().await;
    interact.rewards_reserve().await;
    interact.generate_blocks_until_epoch(15).await;
    interact.claim_rewards(None).await;
    interact.generate_blocks_until_epoch(20).await;
    interact.recompute_token_reserve().await;
    interact.rewards_reserve().await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_delegate_not_enough_egld() {
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact.whitelist_delegation_contract().await;
    interact.set_state_active().await;
    let _ = interact.register_ls_token().await;
    let _ = interact.register_unstake_token().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.generate_blocks_until_epoch(5).await;
    interact.claim_rewards(None).await;
    interact.generate_blocks_until_epoch(10).await;
    interact.recompute_token_reserve().await;
    interact.rewards_reserve().await;
    interact
        .delegate_rewards(Some(ExpectError(
            4,
            "Old claimed rewards must be greater than 1 EGLD",
        )))
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn delegate_vote() {
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact.whitelist_delegation_contract().await;
    interact.set_state_active().await;
    let ls_token = interact.register_ls_token().await;
    let _ = interact.register_unstake_token().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.add_liquidity().await;
    interact.deploy_and_setup_vote_sc().await;
    interact.delegate_vote(&ls_token).await;
}
