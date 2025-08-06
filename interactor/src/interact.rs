use delegation_sc_interact::DelegateCallsInteract;
use governance_sc_interact::GovernanceCallsInteract;
use state::State;

use multiversx_sc_snippets::{imports::*, sdk::gateway::NetworkStatusRequest};

use crate::{state, Config, CHAIN_SIMULATOR_GATEWAY};

pub struct Interact {
    pub interactor: Interactor,
    pub delegation_interactor: Option<DelegateCallsInteract>,
    pub governance_interactor: GovernanceCallsInteract,
    pub wallet_address: Address,
    pub liquid_staking_contract_code: BytesValue,
    pub vote_contract_code: BytesValue,
    pub state: State,
}

impl Interact {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("interactor");

        let governance_interactor =
            GovernanceCallsInteract::new(governance_sc_interact::Config::chain_simulator_config())
                .await;

        let wallet_address = interactor.register_wallet(test_wallets::mallory()).await;

        interactor.generate_blocks_until_epoch(1).await.unwrap();

        let liquid_staking_contract_code = BytesValue::interpret_from(
            "mxsc:../liquid-staking/output/liquid-staking.mxsc.json",
            &InterpreterContext::default(),
        );

        let vote_contract_code = BytesValue::interpret_from(
            "mxsc:../vote-sc/output/vote-sc.mxsc.json",
            &InterpreterContext::default(),
        );

        Interact {
            interactor,
            delegation_interactor: None,
            governance_interactor,
            wallet_address,
            liquid_staking_contract_code,
            vote_contract_code,
            state: State::load_state(),
        }
    }

    pub async fn generate_blocks_until_epoch(&mut self, epoch: u64) {
        self.interactor
            .generate_blocks_until_epoch(epoch)
            .await
            .unwrap();
    }
    pub async fn generate_blocks(&mut self, num_blocks: u64) {
        self.interactor.generate_blocks(num_blocks).await.unwrap();
    }

    pub async fn generate_blocks_until_next_epoch(&mut self) {
        self.interactor
            .generate_blocks_until_epoch(self.get_next_epoch().await)
            .await
            .unwrap();
    }

    async fn get_next_epoch(&self) -> u64 {
        let blockchain = GatewayHttpProxy::new(CHAIN_SIMULATOR_GATEWAY.to_string());

        let network_config = blockchain
            .http_request(NetworkStatusRequest::default())
            .await
            .unwrap();
        network_config.epoch_number + 1u64
    }
}
