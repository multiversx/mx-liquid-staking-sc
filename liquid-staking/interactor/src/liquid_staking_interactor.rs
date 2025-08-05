#![allow(non_snake_case)]

mod contract_interactions;
mod liquid_staking_config;
mod liquid_staking_interact;
mod liquid_staking_state;
mod proxy;
mod vote_proxy;

pub use liquid_staking_config::Config;
pub use liquid_staking_interact::LiquidStakingInteract;
use multiversx_sc_snippets::env_logger;
pub const CHAIN_SIMULATOR_GATEWAY: &str = "http://localhost:8085";

pub async fn liquid_staking_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");

    let config = Config::load_config();
    let mut interact = LiquidStakingInteract::new(config).await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "upgrade" => interact.upgrade().await,
        "addLiquidity" => interact.add_liquidity().await,
        "removeLiquidity" => interact.remove_liquidity("").await,
        "unbondTokens" => interact.unbond_tokens("").await,
        "withdrawAll" => interact.withdraw_all(None).await,
        "claimRewards" => interact.claim_rewards(None).await,
        "recomputeTokenReserve" => interact.recompute_token_reserve().await,
        "delegateRewards" => interact.delegate_rewards(None).await,
        "getLsValueForPosition" => interact.get_ls_value_for_position().await,
        "registerLsToken" => _ = interact.register_ls_token().await,
        "registerUnstakeToken" => _ = interact.register_unstake_token().await,
        "getState" => interact.state().await,
        "getLsTokenId" => interact.ls_token().await,
        "getLsSupply" => interact.ls_token_supply().await,
        "getVirtualEgldReserve" => interact.virtual_egld_reserve().await,
        "getRewardsReserve" => interact.rewards_reserve().await,
        "getUnstakeTokenId" => interact.unstake_token().await,
        "clearOngoingWhitelistOp" => interact.clear_ongoing_whitelist_op().await,
        "whitelistDelegationContract" => interact.whitelist_delegation_contract().await,
        "changeDelegationContractAdmin" => interact.change_delegation_contract_admin().await,
        "changeDelegationContractParams" => interact.change_delegation_contract_params().await,
        "getDelegationStatus" => interact.get_delegation_status().await,
        "getDelegationContractStakedAmount" => {
            interact.get_delegation_contract_staked_amount().await
        }
        "getDelegationContractUnstakedAmount" => {
            interact.get_delegation_contract_unstaked_amount().await
        }
        "getDelegationContractUnbondedAmount" => {
            interact.get_delegation_contract_unbonded_amount().await
        }
        "setStateActive" => interact.set_state_active().await,
        "setStateInactive" => interact.set_state_inactive().await,
        "getDelegationAddressesList" => interact.delegation_addresses_list().await,
        "getAddressesToClaim" => interact.addresses_to_claim().await,
        "getDelegationClaimStatus" => interact.delegation_claim_status().await,
        "getDelegationContractData" => interact.delegation_contract_data().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}
