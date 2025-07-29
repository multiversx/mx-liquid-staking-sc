use clap::{Args, Parser, Subcommand};

pub const PROOF_LENGTH: usize = 18;
pub const HASH_LENGTH: usize = 32;

/// VoteSC Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// VoteSC Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy vote contract")]
    Deploy,
    #[command(name = "set-root-hash", about = "Set the root hash of a proposal")]
    SetRootHash(SetRootHashArgs),
    #[command(
        name = "set-liquid-staking-address",
        about = "Store the address of the Liquid Staking smart contract"
    )]
    SetLiquidStakingAddress(SetLiquidStakingAddressArgs),
    #[command(name = "delegate-vote", about = "Make a delegate vote")]
    DelegateVote(DelegateVoteArgs),
    #[command(name = "get-root-hash", about = "Get the root hash of a proposal")]
    GetRoorHash(GetRootHashArgs),
    #[command(name = "confirm-voting-power", about = "Confirm owned voting power")]
    ConfirmVotingPower(ConfirmVotingPowerArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct SetRootHashArgs {
    #[arg(short = 't', long = "hash")]
    pub root_hash: String,

    #[arg(short = 'f', long = "proposal")]
    pub proposal_id: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct SetLiquidStakingAddressArgs {
    /// Address of the Liquid Staking SC
    #[arg(short = 'n', long = "address")]
    pub address: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegateVoteArgs {
    #[arg(short = 'n', long = "proposal")]
    pub proposal_id: u32,

    #[arg(short = 'n', long = "vote")]
    pub vote: String,

    #[arg(short = 'n', long = "power")]
    pub voting_power: u128,

    #[arg(short = 'n', long = "proof")]
    pub proof: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct GetRootHashArgs {
    /// Automatic activation flag

    #[arg(short = 'f', long = "proposal")]
    pub proposal_id: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ConfirmVotingPowerArgs {
    #[arg(short = 'n', long = "proposal")]
    pub proposal_id: u32,

    #[arg(short = 'n', long = "power")]
    pub voting_power: u128,

    #[arg(short = 'n', long = "proof")]
    pub proof: String,
}
