use clap::{Args, Parser, Subcommand};

/// GovernanceFuncCalls Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// GovernanceFuncCalls Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy")]
    Deploy,

    #[command(name = "upgrade", about = "Upgrade")]
    Upgrade,

    #[command(name = "add-liquidity", about = "Add liquidity")]
    AddLiquidity(CallerAndEgldArgs),

    #[command(name = "remove-liquidity", about = "Remove liquidity")]
    RemoveLiquidity(CallerAndPaymentArgs),

    #[command(name = "unbond-tokens", about = "Unbond tokens")]
    UnbondTokens(CallerAndPaymentArgs),

    #[command(name = "withdraw-all", about = "Withdraw all funds")]
    WithdrawAll(AddressArg),

    #[command(name = "claim-rewards", about = "Claim rewards")]
    ClaimRewards(AddressArg),

    #[command(name = "recompute-token-reserve", about = "Recompute token reserve")]
    RecomputeTokenReserve(AddressArg),

    #[command(name = "delegate-rewards", about = "Delegate rewards")]
    DelegateRewards(AddressArg),

    #[command(
        name = "get-ls-value-for-position",
        about = "Get Liquid Staking value for position"
    )]
    GetLsValueForPosition(EgldArg),

    #[command(name = "register-ls-token", about = "Register Liquid Staking token")]
    RegisterLsToken(RegisterTokenArgs),

    #[command(name = "register-unstake-token", about = "Register unstake token")]
    RegisterUnstakeToken(RegisterTokenArgs),

    #[command(name = "get-state", about = "Get state of the Liquid Staking contract")]
    GetState,

    #[command(
        name = "get-ls-token-id",
        about = "Get the token identifier of the Liquid Staking token"
    )]
    GetLsTokenId,

    #[command(name = "get-ls-supply", about = "Get Liquid STaking supply")]
    GetLsSupply,

    #[command(
        name = "get-virtual-egld-reserve",
        about = "Get virtual Liquid Staking reserver"
    )]
    GetVirtualEgldReserve,

    #[command(name = "get-rewards-reserve", about = "Get rewards reserve")]
    GetRewardsReserve,

    #[command(
        name = "get-unstake-token-id",
        about = "Get the token identifier of the unstake token"
    )]
    GetUnstakeTokenId,

    #[command(
        name = "clear-ongoing-whitelist-operation",
        about = "Clear ongoing whitelist operation"
    )]
    ClearOngoingWhitelistOp,

    #[command(
        name = "whitelist-delegation-contract",
        about = "Whitelist delegation contract"
    )]
    WhitelistDelegationContract(WhitelistDelegationContractArgs),

    #[command(
        name = "change-delegation-contract-admin",
        about = "Change the administrator of the delegation contract"
    )]
    ChangeDelegationContractAdmin(ChangeDelegationContractAdminArgs),

    #[command(
        name = "change-delegation-contract-params",
        about = "Change the parameters of the delegation contract"
    )]
    ChangeDelegationContractParams(ChangeDelegationContractParamsArgs),

    #[command(name = "get-delegation-status", about = "Get delegation status")]
    GetDelegationStatus,

    #[command(
        name = "get-delegation-contract-staked-amount",
        about = "Get the staked amount in the delegation contract"
    )]
    GetDelegationContractStakedAmount(AddressArg),

    #[command(
        name = "get-delegation-contract-unstaked-amount",
        about = "Get the unstaked amount from the delegation contract"
    )]
    GetDelegationContractUnstakedAmount(AddressArg),

    #[command(
        name = "get-delegation-contract-unbonded-contract",
        about = "Get the unbonded amount from the delegation contract"
    )]
    GetDelegationContractUnbondedAmount(AddressArg),

    #[command(
        name = "set-state-active",
        about = "Set the state of the Liquid Staking contract as active"
    )]
    SetStateActive,

    #[command(
        name = "set-state-inactive",
        about = "Set the state of the Liquid Staking contract as inactive"
    )]
    SetStateInactive,

    #[command(
        name = "get-delegation-addresses-list",
        about = "Get the list of delegation contract addresses"
    )]
    GetDelegationAddressesList,

    #[command(name = "get-addresses-to-claim", about = "Get the addresses to claim")]
    GetAddressesToClaim,

    #[command(
        name = "get-delgation-claim-status",
        about = "Get the delegation claim status"
    )]
    GetDelegationClaimStatus,

    #[command(
        name = "get-delegation-contract-data",
        about = "Get the delegation contract data"
    )]
    GetDelegationContractData(AddressArg),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct AddressArg {
    #[arg(short = 'n', long = "address")]
    pub address: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CallerAndEgldArgs {
    #[arg(short = 'n', long = "address")]
    pub caller: String,

    #[arg(short = 'n', long = "egld")]
    pub egld: u128,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct EgldArg {
    #[arg(short = 'n', long = "egld")]
    pub egld: u128,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CallerAndPaymentArgs {
    #[arg(short = 'n', long = "address")]
    pub caller: String,

    #[arg(short = 'n', long = "token")]
    pub token: String,

    #[arg(short = 'n', long = "amount")]
    pub amount: u128,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct RegisterTokenArgs {
    #[arg(short = 'n', long = "token")]
    pub token_display_name: String,

    #[arg(short = 'n', long = "ticker")]
    pub token_ticker: String,

    #[arg(short = 'n', long = "decimals")]
    pub num_decimals: u32,

    #[arg(short = 'n', long = "amount")]
    pub amount: u128,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct WhitelistDelegationContractArgs {
    #[arg(short = 'n', long = "amount")]
    pub amount: u128,

    #[arg(short = 'n', long = "contract")]
    pub contract_address: String,

    #[arg(short = 'n', long = "admin")]
    pub admin_address: String,

    #[arg(short = 'n', long = "staked")]
    pub total_staked: u128,

    #[arg(short = 'n', long = "cap")]
    pub delegation_contract_cap: u128,

    #[arg(short = 'n', long = "nodes")]
    pub nr_nodes: u64,

    #[arg(short = 'n', long = "apy")]
    pub apy: u64,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ChangeDelegationContractAdminArgs {
    #[arg(short = 'n', long = "contract")]
    pub contract_address: String,

    #[arg(short = 'n', long = "admin")]
    pub admin_address: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ChangeDelegationContractParamsArgs {
    #[arg(short = 'n', long = "contract")]
    pub contract_address: String,

    #[arg(short = 'n', long = "staked")]
    pub total_staked: u128,

    #[arg(short = 'n', long = "cap")]
    pub delegation_contract_cap: u128,

    #[arg(short = 'n', long = "nodes")]
    pub nr_nodes: u64,

    #[arg(short = 'n', long = "apy")]
    pub apy: u64,
}
