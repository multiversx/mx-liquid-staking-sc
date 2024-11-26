use liquid_staking_interactor::Config;
use liquid_staking_interactor::ContractInteract;
use multiversx_sc_snippets::imports::*;

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_unbound_tokens_happy_path() {
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
    interact.remove_liquidity(&ls_token).await;
    interact.generate_blocks_until_epoch(14).await;
    interact.unbond_tokens(&us_token).await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn remove_liquidity_and_unbond_early() {
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
    interact.remove_liquidity(&ls_token).await;
    interact
        .unbond_tokens_error(
            &us_token,
            Some(ExpectError(4, "The unstake period has not passed")),
        )
        .await;
}
