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

    #[command(name = "add-liquidity", about = "Add Liquidity")]
    AddLiquidity(CallerAndEgldArgs),

    #[command(name = "remove-liquidity", about = "Remove Liquidity")]
    RemoveLiquidity(CallerAndPaymentArgs),

    #[command(name = "unbond-tokens", about = "Unbond Tokens")]
    UnbondTokens(CallerAndPaymentArgs),

    #[command(name = "withdraw-all", about = "Withdraw all funds")]
    WithdrawAll(AddressArg),

    #[command(name = "", about = "")]
    ClaimRewards(AddressArg),

    #[command(name = "", about = "")]
    RecomputeTokenReserve(AddressArg),

    #[command(name = "", about = "")]
    DelegateRewards(AddressArg),

    #[command(name = "", about = "")]
    GetLsValueForPosition(EgldArg),

    #[command(name = "", about = "")]
    RegisterLsToken(RegisterTokenArgs),

    #[command(name = "", about = "")]
    RegisterUnstakeToken(RegisterTokenArgs),

    #[command(name = "", about = "")]
    GetState,

    #[command(name = "", about = "")]
    GetLsTokenId,

    #[command(name = "", about = "")]
    GetLsSupply,

    #[command(name = "", about = "")]
    GetVirtualEgldReserve,

    #[command(name = "", about = "")]
    GetRewardsReserve,

    #[command(name = "", about = "")]
    GetUnstakeTokenId,

    #[command(name = "", about = "")]
    ClearOngoingWhitelistOp,

    #[command(name = "", about = "")]
    WhitelistDelegationContract(WhitelistDelegationContractArgs),

    #[command(name = "", about = "")]
    ChangeDelegationContractAdmin(ChangeDelegationContractAdminArgs),

    #[command(name = "", about = "")]
    ChangeDelegationContractParams(ChangeDelegationContractParamsArgs),

    #[command(name = "", about = "")]
    GetDelegationStatus,

    #[command(name = "", about = "")]
    GetDelegationContractStakedAmount(AddressArg),

    #[command(name = "", about = "")]
    GetDelegationContractUnstakedAmount(AddressArg),

    #[command(name = "", about = "")]
    GetDelegationContractUnbondedAmount(AddressArg),

    #[command(name = "", about = "")]
    SetStateActive,

    #[command(name = "", about = "")]
    SetStateInactive,

    #[command(name = "", about = "")]
    GetDelegationAddressesList,

    #[command(name = "", about = "")]
    GetAddressesToClaim,

    #[command(name = "", about = "")]
    GetDelegationClaimStatus,

    #[command(name = "", about = "")]
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
