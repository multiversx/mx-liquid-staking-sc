#![allow(non_snake_case)]

mod liquid_staking_config;
mod liquid_staking_state;
mod proxy;

use delegation_sc_interact::{DelegateCallsInteract, DelegationConfig};
pub use liquid_staking_config::Config;
use liquid_staking_state::State;
use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk::gateway::NetworkStatusRequest;

pub const CHAIN_SIMULATOR_GATEWAY: &str = "http://localhost:8085";

pub async fn liquid_staking_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");

    let config = Config::load_config();
    let mut interact = ContractInteract::new(config).await;
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

pub struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State,
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("liquid-staking");
        let wallet_address = interactor.register_wallet(test_wallets::mallory()).await;

        interactor.generate_blocks_until_epoch(1).await.unwrap();

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/liquid-staking.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;

        let new_address_bech32 = Bech32Address::from(&new_address);
        self.state.set_address(new_address_bech32.clone());

        let new_address_string = new_address_bech32.to_string();

        println!("new address: {new_address_string}");
    }

    pub async fn deploy_delegation_contract(&mut self) {
        let mut delegation_interactor =
            DelegateCallsInteract::new(DelegationConfig::chain_simulator_config()).await;
        let validator_1 =
            Validator::from_pem_file("./../../delegation/interactor/validatorKey1.pem")
                .expect("unable to load validator key");
        let validator_2 =
            Validator::from_pem_file("./../../delegation/interactor/validatorKey2.pem")
                .expect("unable to load validator key");

        let _ = delegation_interactor
            .interactor
            .add_key(validator_1.private_key.clone())
            .await
            .unwrap();
        let _ = delegation_interactor
            .interactor
            .add_key(validator_2.private_key.clone())
            .await
            .unwrap();

        delegation_interactor
            .set_state(&delegation_interactor.wallet_address.to_address())
            .await;
        delegation_interactor
            .set_state(&delegation_interactor.delegator1.to_address())
            .await;
        delegation_interactor
            .set_state(&delegation_interactor.delegator2.to_address())
            .await;
        delegation_interactor
            .create_new_delegation_contract(51_000_000_000_000_000_000_000_u128, 3745u64)
            .await;
        delegation_interactor
            .set_check_cap_on_redelegate_rewards(false)
            .await;

        let addresses = delegation_interactor.get_all_contract_addresses().await;

        let new_address_bech32 = &addresses[0];
        self.state
            .set_delegation_address(new_address_bech32.clone());

        let new_address_string = new_address_bech32.to_string();
        println!("new delegation address: {new_address_string}");
    }

    pub async fn upgrade(&mut self) {
        let response = self
            .interactor
            .tx()
            .to(self.state.current_address())
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .upgrade()
            .code(&self.contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewAddress)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_liquidity(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(1_000_000_000_000_000_001u64);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .add_liquidity()
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn remove_liquidity(&mut self, token_id: &str) {
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(1_000_000_000_000_000_000u64);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .remove_liquidity()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn unbond_tokens(&mut self, token_id: &str) {
        let token_nonce = 1u64;
        let token_amount = BigUint::<StaticApi>::from(1u64);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .unbond_tokens()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn withdraw_all(&mut self, error: Option<ExpectError<'_>>) {
        let delegation_contract = self.state.delegation_address();

        let tx = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .withdraw_all(delegation_contract);

        match error {
            None => {
                tx.returns(ReturnsResultUnmanaged).run().await;
            }
            Some(expect_error) => {
                tx.returns(expect_error).run().await;
            }
        }
    }

    pub async fn claim_rewards(&mut self, error: Option<ExpectError<'_>>) {
        let tx = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .claim_rewards();

        match error {
            None => {
                tx.returns(ReturnsResultUnmanaged).run().await;
            }
            Some(expect_error) => {
                tx.returns(expect_error).run().await;
            }
        }
    }

    pub async fn recompute_token_reserve(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .recompute_token_reserve()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn delegate_rewards(&mut self, error: Option<ExpectError<'_>>) {
        let tx = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .delegate_rewards();

        match error {
            None => {
                tx.returns(ReturnsResultUnmanaged).run().await;
            }
            Some(expect_error) => {
                tx.returns(expect_error).run().await;
            }
        }
    }

    pub async fn get_ls_value_for_position(&mut self) {
        let ls_token_amount = BigUint::<StaticApi>::from(0u128);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .get_ls_value_for_position(ls_token_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn register_ls_token(&mut self) -> String {
        let egld_amount = BigUint::<StaticApi>::from(50_000_000_000_000_000u64);

        let token_display_name = ManagedBuffer::new_from_bytes(&b"LIQTEST"[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b"LTST"[..]);
        let num_decimals = 18u32;

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(90_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .register_ls_token(token_display_name, token_ticker, num_decimals)
            .egld(egld_amount)
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await
    }

    pub async fn register_unstake_token(&mut self) -> String {
        let egld_amount = BigUint::<StaticApi>::from(50_000_000_000_000_000u64);

        let token_display_name = ManagedBuffer::new_from_bytes(&b"UNSTAKETEST"[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b"UNTST"[..]);
        let num_decimals = 18u32;

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(90_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .register_unstake_token(token_display_name, token_ticker, num_decimals)
            .egld(egld_amount)
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await
    }

    pub async fn state(&mut self) {
        self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .state()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn ls_token(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .ls_token()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn ls_token_supply(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .ls_token_supply()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn virtual_egld_reserve(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .virtual_egld_reserve()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn rewards_reserve(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .rewards_reserve()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn unstake_token(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .unstake_token()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn clear_ongoing_whitelist_op(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .clear_ongoing_whitelist_op()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn whitelist_delegation_contract(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(1_000_000_000_000_000_000u64);

        let contract_address = self.state.delegation_address();
        let admin_address = &self.wallet_address;
        let total_staked = BigUint::<StaticApi>::from(0u128);
        let delegation_contract_cap = BigUint::<StaticApi>::from(5_000_000_000_000_000_000u64);
        let nr_nodes = 1u64;
        let apy = 50_000u64;

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .whitelist_delegation_contract(
                contract_address,
                admin_address,
                total_staked,
                delegation_contract_cap,
                nr_nodes,
                apy,
            )
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn change_delegation_contract_admin(&mut self) {
        let contract_address = self.state.delegation_address();
        let admin_address = Bech32Address::from_bech32_string("".to_string());

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .change_delegation_contract_admin(contract_address, admin_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn change_delegation_contract_params(&mut self) {
        let contract_address = self.state.delegation_address();
        let total_staked = BigUint::<StaticApi>::from(0u128);
        let delegation_contract_cap = BigUint::<StaticApi>::from(0u128);
        let nr_nodes = 0u64;
        let apy = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .change_delegation_contract_params(
                contract_address,
                total_staked,
                delegation_contract_cap,
                nr_nodes,
                apy,
            )
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_delegation_status(&mut self) {
        self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .get_delegation_status()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn get_delegation_contract_staked_amount(&mut self) {
        let delegation_address = self.state.delegation_address();

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .get_delegation_contract_staked_amount(delegation_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_delegation_contract_unstaked_amount(&mut self) {
        let delegation_address = self.state.delegation_address();

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .get_delegation_contract_unstaked_amount(delegation_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_delegation_contract_unbonded_amount(&mut self) {
        let delegation_address = self.state.delegation_address();

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .get_delegation_contract_unbonded_amount(delegation_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn set_state_active(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .set_state_active()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn set_state_inactive(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .set_state_inactive()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn delegation_addresses_list(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .delegation_addresses_list()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn addresses_to_claim(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .addresses_to_claim()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_voting_power(&mut self, token_id: &str, token_amount: BigUint<StaticApi>) {
        let token_nonce = 0u64;

        let payment =
            EsdtTokenPayment::new(TokenIdentifier::from(token_id), token_nonce, token_amount);

        self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .get_voting_power(payment)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn delegation_claim_status(&mut self) {
        self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .delegation_claim_status()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn delegation_contract_data(&mut self) {
        let contract_address = self.state.delegation_address();

        self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .delegation_contract_data(contract_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn generate_blocks_until_epoch(&mut self, epoch: u64) {
        self.interactor
            .generate_blocks_until_epoch(epoch)
            .await
            .unwrap();
    }

    pub async fn deploy_governance_contract(&mut self) {
        // let mut governance_interactor =
        //     GovernanceCallsInteract::new(Config::chain_simulator_config()).await;

        // governance_interactor
        //     .set_state(&governance_interactor.owner.to_address())
        //     .await;

        // governance_interactor
        //     .proposal("6db132d759482f9f3515fe3ca8f72a8d6dc61244", 9, 11)
        //     .await;

        // let current_epoch = Self::get_current_epoch().await;
        // self.interactor
        //     .tx()
        //     .from(&self.wallet_address)
        //     .to(governance_sc_address)
        //     .typed(GovernanceSCProxy)
        //     .proposal(b"play chess", current_epoch, current_epoch + 5)
        //     .returns(ReturnsResultUnmanaged)
        //     .run()
        //     .await;

        // self.interactor
        //     .generate_blocks_until_epoch(5)
        //     .await
        //     .unwrap();

        // self.state
        //     .set_governance_address(new_address_bech32.clone());

        // println!("vote sc new address: {new_address_bech32}");
    }

    #[allow(dead_code)]
    async fn get_current_epoch() -> u64 {
        let blockchain = GatewayHttpProxy::new(CHAIN_SIMULATOR_GATEWAY.to_string());

        let network_config = blockchain
            .http_request(NetworkStatusRequest::default())
            .await
            .unwrap();
        network_config.epoch_number
    }
}
