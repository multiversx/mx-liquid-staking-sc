#![allow(non_snake_case)]

mod config;
mod contract_interactions;
mod contract_proxies;
mod interact;
mod interact_cli;
mod state;

use clap::Parser;
pub use config::Config;
pub use interact::Interact;
use multiversx_sc::imports::Bech32Address;
use multiversx_sc_snippets::env_logger;
pub const CHAIN_SIMULATOR_GATEWAY: &str = "http://localhost:8085";

pub async fn liquid_staking_cli() {
    env_logger::init();

    let mut interact = Interact::new(Config::load_config()).await;

    let cli = interact_cli::InteractCli::parse();

    match cli.command {
        Some(interact_cli::InteractCliCommand::Deploy) => interact.deploy().await,
        Some(interact_cli::InteractCliCommand::Upgrade) => interact.upgrade().await,
        Some(interact_cli::InteractCliCommand::AddLiquidity(args)) => {
            interact
                .add_liquidity(Bech32Address::from_bech32_string(args.caller), args.egld)
                .await
        }
        Some(interact_cli::InteractCliCommand::RemoveLiquidity(args)) => {
            interact
                .remove_liquidity(
                    Bech32Address::from_bech32_string(args.caller),
                    &args.token,
                    args.amount,
                )
                .await
        }
        Some(interact_cli::InteractCliCommand::UnbondTokens(args)) => {
            interact
                .unbond_tokens(
                    Bech32Address::from_bech32_string(args.caller),
                    &args.token,
                    args.amount,
                )
                .await
        }
        Some(interact_cli::InteractCliCommand::WithdrawAll(args)) => {
            interact
                .withdraw_all(Bech32Address::from_bech32_string(args.address), None)
                .await
        }
        Some(interact_cli::InteractCliCommand::ClaimRewards(args)) => {
            interact
                .claim_rewards(Bech32Address::from_bech32_string(args.address), None)
                .await
        }
        Some(interact_cli::InteractCliCommand::RecomputeTokenReserve(args)) => {
            interact
                .recompute_token_reserve(Bech32Address::from_bech32_string(args.address))
                .await
        }
        Some(interact_cli::InteractCliCommand::DelegateRewards(args)) => {
            interact
                .delegate_rewards(Bech32Address::from_bech32_string(args.address), None)
                .await
        }
        Some(interact_cli::InteractCliCommand::GetLsValueForPosition(args)) => {
            interact.get_ls_value_for_position(args.egld).await
        }
        Some(interact_cli::InteractCliCommand::RegisterLsToken(args)) => {
            _ = interact
                .register_ls_token(
                    &args.token_display_name,
                    &args.token_ticker,
                    args.num_decimals,
                    args.amount,
                )
                .await
        }
        Some(interact_cli::InteractCliCommand::RegisterUnstakeToken(args)) => {
            _ = interact
                .register_unstake_token(
                    &args.token_display_name,
                    &args.token_ticker,
                    args.num_decimals,
                    args.amount,
                )
                .await
        }
        Some(interact_cli::InteractCliCommand::GetState) => interact.state().await,
        Some(interact_cli::InteractCliCommand::GetLsTokenId) => interact.ls_token().await,
        Some(interact_cli::InteractCliCommand::GetLsSupply) => interact.ls_token_supply().await,
        Some(interact_cli::InteractCliCommand::GetVirtualEgldReserve) => {
            interact.virtual_egld_reserve().await
        }
        Some(interact_cli::InteractCliCommand::GetRewardsReserve) => {
            interact.rewards_reserve().await
        }
        Some(interact_cli::InteractCliCommand::GetUnstakeTokenId) => interact.unstake_token().await,
        Some(interact_cli::InteractCliCommand::ClearOngoingWhitelistOp) => {
            interact.clear_ongoing_whitelist_op().await
        }
        Some(interact_cli::InteractCliCommand::WhitelistDelegationContract(args)) => {
            interact
                .whitelist_delegation_contract(
                    args.amount,
                    Bech32Address::from_bech32_string(args.contract_address),
                    Bech32Address::from_bech32_string(args.admin_address),
                    args.total_staked,
                    args.delegation_contract_cap,
                    args.nr_nodes,
                    args.apy,
                )
                .await
        }
        Some(interact_cli::InteractCliCommand::ChangeDelegationContractAdmin(args)) => {
            interact
                .change_delegation_contract_admin(
                    Bech32Address::from_bech32_string(args.admin_address),
                    Bech32Address::from_bech32_string(args.contract_address),
                )
                .await
        }
        Some(interact_cli::InteractCliCommand::ChangeDelegationContractParams(args)) => {
            interact
                .change_delegation_contract_params(
                    Bech32Address::from_bech32_string(args.contract_address),
                    args.total_staked,
                    args.delegation_contract_cap,
                    args.nr_nodes,
                    args.apy,
                )
                .await
        }
        Some(interact_cli::InteractCliCommand::GetDelegationStatus) => {
            interact.get_delegation_status().await
        }
        Some(interact_cli::InteractCliCommand::GetDelegationContractStakedAmount(args)) => {
            interact
                .get_delegation_contract_staked_amount(Bech32Address::from_bech32_string(
                    args.address,
                ))
                .await
        }
        Some(interact_cli::InteractCliCommand::GetDelegationContractUnstakedAmount(args)) => {
            interact
                .get_delegation_contract_unstaked_amount(Bech32Address::from_bech32_string(
                    args.address,
                ))
                .await
        }
        Some(interact_cli::InteractCliCommand::GetDelegationContractUnbondedAmount(args)) => {
            interact
                .get_delegation_contract_unbonded_amount(Bech32Address::from_bech32_string(
                    args.address,
                ))
                .await
        }
        Some(interact_cli::InteractCliCommand::SetStateActive) => interact.set_state_active().await,
        Some(interact_cli::InteractCliCommand::SetStateInactive) => {
            interact.set_state_inactive().await
        }
        Some(interact_cli::InteractCliCommand::GetDelegationAddressesList) => {
            interact.delegation_addresses_list().await
        }
        Some(interact_cli::InteractCliCommand::GetAddressesToClaim) => {
            interact.addresses_to_claim().await
        }
        Some(interact_cli::InteractCliCommand::GetDelegationClaimStatus) => {
            interact.delegation_claim_status().await
        }
        Some(interact_cli::InteractCliCommand::GetDelegationContractData(args)) => {
            interact
                .delegation_contract_data(Bech32Address::from_bech32_string(args.address))
                .await
        }
        None => {}
    }
}
