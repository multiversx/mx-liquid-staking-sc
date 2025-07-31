#![allow(non_snake_case)]

pub mod vote_interact_cli;
pub mod vote_interact_config;
mod vote_proxy;
pub mod vote_state;

use clap::Parser;
use multiversx_sc_snippets::imports::*;
use vote_interact_config::Config;
use vote_sc::constants::PROOF_LENGTH;
use vote_state::State;

use crate::vote_interact_cli::HASH_LENGTH;

pub async fn vote_sc_cli() {
    env_logger::init();

    let mut interact = VoteInteract::new(Config::load_config()).await;

    let cli = vote_interact_cli::InteractCli::parse();
    match cli.command {
        Some(vote_interact_cli::InteractCliCommand::Deploy) => interact.deploy().await,
        Some(vote_interact_cli::InteractCliCommand::SetRootHash(args)) => {
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(args.root_hash.as_bytes());
            let root_hash = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(&bytes);
            interact.set_root_hash(root_hash, args.proposal_id).await
        }
        Some(vote_interact_cli::InteractCliCommand::SetLiquidStakingAddress(args)) => {
            interact
                .set_liquid_staking_address(Bech32Address::from_bech32_string(args.address))
                .await
        }
        Some(vote_interact_cli::InteractCliCommand::DelegateVote(args)) => {
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(args.proof.as_bytes());
            let hash = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(&bytes);
            let mut proof = ArrayVec::new();
            proof.push(hash);

            interact
                .delegate_vote(args.proposal_id, &args.vote, args.voting_power, proof)
                .await
        }
        Some(vote_interact_cli::InteractCliCommand::GetRoorHash(args)) => {
            interact.get_root_hash(args.proposal_id).await
        }
        Some(vote_interact_cli::InteractCliCommand::ConfirmVotingPower(args)) => {
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(args.proof.as_bytes());
            let hash = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(&bytes);
            let mut proof = ArrayVec::new();
            proof.push(hash);

            interact
                .confirm_voting_power(args.proposal_id, args.voting_power, proof)
                .await
        }
        None => {}
    }
}

pub struct VoteInteract {
    pub interactor: Interactor,
    pub owner: Bech32Address,
    pub contract_code: BytesValue,
    pub state: State,
}

impl VoteInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("fuck-you-relative-pathing");
        let wallet_address = interactor.register_wallet(test_wallets::mallory()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/vote-sc.mxsc.json",
            &InterpreterContext::default(),
        );

        Self {
            interactor,
            owner: wallet_address.into(),
            contract_code,
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.owner)
            .gas(30_000_000u64)
            .typed(vote_proxy::VoteSCProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = new_address.to_bech32_default();
        println!("new address: {new_address_bech32}");
        self.state.set_vote_address(new_address_bech32);
    }

    pub async fn set_root_hash(
        &mut self,
        root_hash: ManagedByteArray<StaticApi, { HASH_LENGTH }>,
        proposal_id: u32,
    ) {
        let response = self
            .interactor
            .tx()
            .from(&self.owner)
            .to(self.state.current_vote_address())
            .gas(30_000_000u64)
            .typed(vote_proxy::VoteSCProxy)
            .set_root_hash(root_hash, proposal_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_liquid_staking_address(&mut self, address: Bech32Address) {
        let response = self
            .interactor
            .tx()
            .from(&self.owner)
            .to(self.state.current_vote_address())
            .gas(30_000_000u64)
            .typed(vote_proxy::VoteSCProxy)
            .set_liquid_staking_address(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn delegate_vote(
        &mut self,
        proposal_id: u32,
        vote: &str,
        voting_power: u128,
        proof: ArrayVec<ManagedByteArray<StaticApi, { HASH_LENGTH }>, { PROOF_LENGTH }>,
    ) {
        let response = self
            .interactor
            .tx()
            .from(&self.owner)
            .to(self.state.current_vote_address())
            .gas(30_000_000u64)
            .typed(vote_proxy::VoteSCProxy)
            .delegate_vote(proposal_id, vote, voting_power, proof)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_root_hash(&mut self, proposal_id: u32) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_vote_address())
            .typed(vote_proxy::VoteSCProxy)
            .get_root_hash(proposal_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn confirm_voting_power(
        &mut self,
        proposal_id: u32,
        voting_power: u128,
        proof: ArrayVec<ManagedByteArray<StaticApi, { HASH_LENGTH }>, { PROOF_LENGTH }>,
    ) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_vote_address())
            .typed(vote_proxy::VoteSCProxy)
            .confirm_voting_power(proposal_id, voting_power, proof)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }
}
