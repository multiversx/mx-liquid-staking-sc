use delegation_sc_interact::DelegateCallsInteract;
use governance_sc_interact::GovernanceCallsInteract;
use liquid_staking_state::State;

use multiversx_sc_snippets::imports::*;
use vote_interact::{vote_interact_config, VoteInteract};

use crate::{liquid_staking_state, Config};

pub struct LiquidStakingInteract {
    pub(crate) interactor: Interactor,
    pub(crate) delegation_interactor: Option<DelegateCallsInteract>,
    pub(crate) governance_interactor: GovernanceCallsInteract,
    pub(crate) vote_interactor: VoteInteract,
    pub(crate) wallet_address: Address,
    pub(crate) liquid_staking_contract_code: BytesValue,
    pub(crate) state: State,
}

impl LiquidStakingInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        let vote_interactor =
            VoteInteract::new(vote_interact_config::Config::chain_simulator_config()).await;

        let governance_interactor =
            GovernanceCallsInteract::new(governance_sc_interact::Config::chain_simulator_config())
                .await;

        let wallet_address = interactor.register_wallet(test_wallets::mallory()).await;

        interactor.generate_blocks_until_epoch(1).await.unwrap();

        let liquid_staking_contract_code = BytesValue::interpret_from(
            "mxsc:../output/liquid-staking.mxsc.json",
            &InterpreterContext::default(),
        );

        LiquidStakingInteract {
            interactor,
            delegation_interactor: None,
            governance_interactor,
            vote_interactor,
            wallet_address,
            liquid_staking_contract_code,
            state: State::load_state(),
        }
    }

    pub async fn generate_blocks_until_epoch(&mut self, epoch: u64) {
        self.interactor
            .generate_blocks_until_epoch(epoch)
            .await
            .unwrap();
    }
}
