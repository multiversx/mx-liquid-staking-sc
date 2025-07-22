mod governance_sc_interact_cli;
mod governance_sc_interact_config;
mod governance_sc_interact_state;

use clap::Parser;
pub use governance_sc_interact_config::GovernanceConfig;
use governance_sc_interact_state::State;

use multiversx_sc_snippets::{imports::*, sdk::gateway::SetStateAccount};

pub async fn governance_sc_interact_cli() {
    env_logger::init();

    let mut interactor = GovernanceCallsInteract::new(GovernanceConfig::load_config()).await;

    let cli = governance_sc_interact_cli::InteractCli::parse();
    match cli.command {
        Some(governance_sc_interact_cli::InteractCliCommand::Propose) => {
            interactor.proposal_hardcoded().await;
        }
        Some(governance_sc_interact_cli::InteractCliCommand::ViewConfig) => {
            interactor.view_config().await;
        }
        Some(governance_sc_interact_cli::InteractCliCommand::ViewProposal) => {
            interactor.view_proposal(2).await;
        }
        Some(governance_sc_interact_cli::InteractCliCommand::Vote) => {
            interactor.vote_hardcoded().await;
        }
        None => {}
    }
}

pub struct GovernanceCallsInteract {
    pub interactor: Interactor,
    pub owner: Bech32Address,
    pub user1: Bech32Address,
    pub user2: Bech32Address,
    pub state: State,
}

impl GovernanceCallsInteract {
    pub async fn new(config: GovernanceConfig) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.is_chain_simulator());

        interactor.set_current_dir_from_workspace("tools/interactor-governance-func-calls");
        let owner = interactor.register_wallet(test_wallets::eve()).await;
        let user1 = interactor.register_wallet(test_wallets::mike()).await;
        let user2 = interactor.register_wallet(test_wallets::judy()).await;

        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        Self {
            interactor,
            owner: owner.into(),
            user1: user1.into(),
            user2: user2.into(),
            state: State::load_state(),
        }
    }

    pub async fn set_state(&mut self, address: &Address) {
        let mut account = self.interactor.get_account(address).await;
        account.balance = "100000000000000000000000".to_owned();
        let set_state_account = SetStateAccount::from(account);
        let vec_state = vec![set_state_account];

        let _set_state_response = self.interactor.set_state(vec_state).await;
    }

    pub async fn view_config(&mut self) {
        let raw = self
            .interactor
            .query()
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .view_config()
            .returns(ReturnsRawResult)
            .run()
            .await;

        println!("config raw: {:?}", raw);
    }

    pub async fn proposal_hardcoded(&mut self) {
        self.proposal("a1075ebe040351a8a6b457176a253d410edd391c", 4041, 4041)
            .await;
    }

    pub async fn proposal(
        &mut self,
        commit_hash: &str,
        start_vote_epoch: usize,
        end_vote_epoch: usize,
    ) {
        let raw = self
            .interactor
            .tx()
            .from(&self.owner)
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .proposal(commit_hash, start_vote_epoch, end_vote_epoch)
            .gas(60_000_000u64)
            .returns(ReturnsRawResult)
            .run()
            .await;

        println!("proposal result raw: {:?}", raw);
    }

    pub async fn view_proposal(&mut self, nonce: u64) {
        let raw = self
            .interactor
            .query()
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .view_proposal(nonce)
            .returns(ReturnsRawResult)
            .run()
            .await;

        let result_strings = raw
            .into_iter()
            .map(|mb| String::from_utf8_lossy(&mb.to_vec()).into_owned())
            .collect::<Vec<_>>();

        println!("proposal raw: {:?}", result_strings);
    }

    pub async fn vote(&mut self, sender: &Bech32Address, proposal: &str, vote_type: &str) {
        self.interactor
            .tx()
            .from(sender)
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .vote(proposal, vote_type)
            .gas(60_000_000u64)
            .run()
            .await;
    }

    /// Temporary, some hardcoded values for quicker testing.
    pub async fn vote_hardcoded(&mut self) {
        let user_address = self
            .interactor
            .register_wallet(Wallet::from_pem_file("delegator2.pem").unwrap())
            .await;
        self.vote(
            &Bech32Address::encode_address_default_hrp(user_address),
            "2",
            "yes",
        )
        .await;
    }

    pub async fn delegate_vote(
        &mut self,
        sender: &Bech32Address,
        proposal: &str,
        vote: &str,
        voter: &Bech32Address,
        stake: u64,
    ) {
        self.interactor
            .tx()
            .from(sender)
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .delegate_vote(proposal, vote, voter, stake)
            .gas(60_000_000u64)
            .run()
            .await;

        println!("Delegate vote successfully done!");
    }
}
