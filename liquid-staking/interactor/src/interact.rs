#![allow(non_snake_case)]

mod config;
mod delegation_proxy;
mod proxy;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

// INFO: This file contains chain simulator tests and devnet tests. Based on which type of tests you want to run, uncomment the appropriate lines in config.toml.

const STATE_FILE: &str = "state.toml";
const DELEGATION_MOCK_CONTRACT_CODE: &str = "../delegation-mock/output/delegation-mock.mxsc.json";
const LP_TOKEN_ID: &str = "LTST-37287f";
const UNSTAKE_TOKEN_ID: &str = "UNTST-1e3db7";

pub async fn liquid_staking_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new().await;
    match cmd.as_str() {
        "deploy" => interact.deploy(false).await,
        "upgrade" => interact.upgrade().await,
        "addLiquidity" => interact.add_liquidity(false).await,
        "removeLiquidity" => interact.remove_liquidity(false).await,
        "unbondTokens" => interact.unbond_tokens(false).await,
        "withdrawAll" => interact.withdraw_all().await,
        "claimRewards" => interact.claim_rewards().await,
        "recomputeTokenReserve" => interact.recompute_token_reserve().await,
        "delegateRewards" => interact.delegate_rewards().await,
        "getLsValueForPosition" => interact.get_ls_value_for_position().await,
        "registerLsToken" => interact.register_ls_token(false).await,
        "registerUnstakeToken" => interact.register_unstake_token(false).await,
        "getState" => interact.state().await,
        "getLsTokenId" => interact.ls_token().await,
        "getLsSupply" => interact.ls_token_supply().await,
        "getVirtualEgldReserve" => interact.virtual_egld_reserve().await,
        "getRewardsReserve" => interact.rewards_reserve().await,
        "getUnstakeTokenId" => interact.unstake_token().await,
        "clearOngoingWhitelistOp" => interact.clear_ongoing_whitelist_op().await,
        "whitelistDelegationContract" => interact.whitelist_delegation_contract(false).await,
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
        "setStateActive" => interact.set_state_active(false).await,
        "setStateInactive" => interact.set_state_inactive().await,
        "getDelegationAddressesList" => interact.delegation_addresses_list().await,
        "getAddressesToClaim" => interact.addresses_to_claim().await,
        "getDelegationClaimStatus" => interact.delegation_claim_status().await,
        "getDelegationContractData" => interact.delegation_contract_data().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>,
    delegation_address: Option<Bech32Address>,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    /// Sets the contract address
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    pub fn set_delegation_address(&mut self, address: Bech32Address) {
        self.delegation_address = Some(address);
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }

    pub fn delegation_address(&self) -> &Bech32Address {
        self.delegation_address
            .as_ref()
            .expect("no known delegation contract, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

pub struct ContractInteract {
    interactor: Interactor,
    chain_simulator: Interactor,
    wallet_address: Address,
    chain_sim_wallet_address: Address,
    contract_code: BytesValue,
    delegation_mock_contract_code: String,
    state: State,
}

impl ContractInteract {
    pub async fn new() -> Self {
        let config = Config::new();
        let mut interactor = Interactor::new(config.gateway_uri()).await;
        let mut chain_simulator = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("liquid-staking");
        chain_simulator.set_current_dir_from_workspace("liquid-staking");
        let wallet_address = interactor.register_wallet(test_wallets::mallory()).await;
        let chain_sim_wallet_address = chain_simulator
            .register_wallet(test_wallets::mallory())
            .await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        chain_simulator
            .generate_blocks_until_epoch(1)
            .await
            .unwrap();

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/liquid-staking.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            chain_simulator,
            wallet_address,
            chain_sim_wallet_address,
            contract_code,
            delegation_mock_contract_code: DELEGATION_MOCK_CONTRACT_CODE.to_string(),
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self, is_chain_simulator: bool) {
        let new_address = match is_chain_simulator {
            true => {
                self.chain_simulator
                    .tx()
                    .from(&self.chain_sim_wallet_address)
                    .gas(90_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .init()
                    .code(&self.contract_code)
                    .returns(ReturnsNewAddress)
                    .run()
                    .await
            }
            false => {
                self.interactor
                    .tx()
                    .from(&self.wallet_address)
                    .gas(90_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .init()
                    .code(&self.contract_code)
                    .returns(ReturnsNewAddress)
                    .run()
                    .await
            }
        };

        let new_address_bech32 = bech32::encode(&new_address);
        self.state.set_address(Bech32Address::from_bech32_string(
            new_address_bech32.clone(),
        ));

        println!("new address: {new_address_bech32}");
    }

    pub async fn deploy_delegation_contract(&mut self, is_chain_simulator: bool) {
        let contract_code = MxscPath::new(&self.delegation_mock_contract_code);

        let new_address = match is_chain_simulator {
            true => {
                self.chain_simulator
                    .tx()
                    .from(&self.chain_sim_wallet_address)
                    .gas(90_000_000u64)
                    .typed(delegation_proxy::DelegationMockProxy)
                    .init()
                    .code(contract_code)
                    .returns(ReturnsNewAddress)
                    .run()
                    .await
            }
            false => {
                self.interactor
                    .tx()
                    .from(&self.wallet_address)
                    .gas(90_000_000u64)
                    .typed(delegation_proxy::DelegationMockProxy)
                    .init()
                    .code(contract_code)
                    .returns(ReturnsNewAddress)
                    .run()
                    .await
            }
        };

        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_delegation_address(Bech32Address::from_bech32_string(
                new_address_bech32.clone(),
            ));

        println!("new address: {new_address_bech32}");
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

    pub async fn add_liquidity(&mut self, is_chain_simulator: bool) {
        let egld_amount = BigUint::<StaticApi>::from(1_000_000_000_000_000_001u64);

        match is_chain_simulator {
            true => {
                self.chain_simulator
                    .tx()
                    .from(&self.chain_sim_wallet_address)
                    .to(self.state.current_address())
                    .gas(50_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .add_liquidity()
                    .egld(egld_amount)
                    .returns(ReturnsResultUnmanaged)
                    .run()
                    .await
            }
            false => {
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
        };
    }

    pub async fn remove_liquidity(&mut self, is_chain_simulator: bool) {
        let token_id = LP_TOKEN_ID;
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(1_000_000_000_000_000_000u64);

        match is_chain_simulator {
            true => {
                self.chain_simulator
                    .tx()
                    .from(&self.chain_sim_wallet_address)
                    .to(self.state.current_address())
                    .gas(50_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .remove_liquidity()
                    .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
                    .returns(ReturnsResultUnmanaged)
                    .run()
                    .await
            }
            false => {
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
        };
    }

    pub async fn unbond_tokens(&mut self, is_chain_simulator: bool) {
        let token_id = UNSTAKE_TOKEN_ID;
        let token_nonce = 1u64;
        let token_amount = BigUint::<StaticApi>::from(1u64);

        match is_chain_simulator {
            true => {
                self.chain_simulator
                    .tx()
                    .from(&self.chain_sim_wallet_address)
                    .to(self.state.current_address())
                    .gas(30_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .unbond_tokens()
                    .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
                    .returns(ReturnsResultUnmanaged)
                    .run()
                    .await;
            }
            false => {
                self.interactor
                    .tx()
                    .from(&self.wallet_address)
                    .to(self.state.current_address())
                    .gas(30_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .unbond_tokens()
                    .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
                    .returns(ReturnsResultUnmanaged)
                    .run()
                    .await;
            }
        };
    }

    pub async fn unbond_tokens_error(&mut self, is_chain_simulator: bool, error: ExpectError<'_>) {
        let token_id = "UNTST-f4f477";
        let token_nonce = 1u64;
        let token_amount = BigUint::<StaticApi>::from(1u64);

        match is_chain_simulator {
            true => {
                self.chain_simulator
                    .tx()
                    .from(&self.chain_sim_wallet_address)
                    .to(self.state.current_address())
                    .gas(30_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .unbond_tokens()
                    .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
                    .returns(error)
                    .run()
                    .await;
            }
            false => {
                self.interactor
                    .tx()
                    .from(&self.wallet_address)
                    .to(self.state.current_address())
                    .gas(30_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .unbond_tokens()
                    .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
                    .returns(error)
                    .run()
                    .await;
            }
        };
    }

    pub async fn withdraw_all(&mut self) {
        let delegation_contract = self.state.delegation_address.as_ref().unwrap();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .withdraw_all(delegation_contract)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn claim_rewards(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.chain_sim_wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .claim_rewards()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn recompute_token_reserve(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.chain_sim_wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .recompute_token_reserve()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn delegate_rewards(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.chain_sim_wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .delegate_rewards()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
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

    pub async fn register_ls_token(&mut self, is_chain_simulator: bool) {
        let egld_amount = BigUint::<StaticApi>::from(50_000_000_000_000_000u64);

        let token_display_name = ManagedBuffer::new_from_bytes(&b"LIQTEST"[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b"LTST"[..]);
        let num_decimals = 18u32;

        match is_chain_simulator {
            true => {
                self.chain_simulator
                    .tx()
                    .from(&self.chain_sim_wallet_address)
                    .to(self.state.current_address())
                    .gas(90_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .register_ls_token(token_display_name, token_ticker, num_decimals)
                    .egld(egld_amount)
                    .returns(ReturnsResultUnmanaged)
                    .run()
                    .await
            }
            false => {
                self.interactor
                    .tx()
                    .from(&self.wallet_address)
                    .to(self.state.current_address())
                    .gas(90_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .register_ls_token(token_display_name, token_ticker, num_decimals)
                    .egld(egld_amount)
                    .returns(ReturnsResultUnmanaged)
                    .run()
                    .await
            }
        };
    }

    pub async fn register_unstake_token(&mut self, is_chain_simulator: bool) {
        let egld_amount = BigUint::<StaticApi>::from(50_000_000_000_000_000u64);

        let token_display_name = ManagedBuffer::new_from_bytes(&b"UNSTAKETEST"[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b"UNTST"[..]);
        let num_decimals = 18u32;

        match is_chain_simulator {
            true => {
                self.chain_simulator
                    .tx()
                    .from(&self.chain_sim_wallet_address)
                    .to(self.state.current_address())
                    .gas(90_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .register_unstake_token(token_display_name, token_ticker, num_decimals)
                    .egld(egld_amount)
                    .returns(ReturnsResultUnmanaged)
                    .run()
                    .await
            }
            false => {
                self.interactor
                    .tx()
                    .from(&self.wallet_address)
                    .to(self.state.current_address())
                    .gas(90_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .register_unstake_token(token_display_name, token_ticker, num_decimals)
                    .egld(egld_amount)
                    .returns(ReturnsResultUnmanaged)
                    .run()
                    .await;
            }
        };
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

    pub async fn whitelist_delegation_contract(&mut self, is_chain_simulator: bool) {
        let egld_amount = BigUint::<StaticApi>::from(1_000_000_000_000_000_000u64);

        let contract_address = self.state.delegation_address.as_ref().unwrap();
        let admin_address = &self.wallet_address;
        let total_staked = BigUint::<StaticApi>::from(0u128);
        let delegation_contract_cap = BigUint::<StaticApi>::from(5_000_000_000_000_000_000u64);
        let nr_nodes = 0u64;
        let apy = 10_000u64;

        match is_chain_simulator {
            true => {
                self.chain_simulator
                    .tx()
                    .from(&self.chain_sim_wallet_address)
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
            false => {
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
        };
    }

    pub async fn change_delegation_contract_admin(&mut self) {
        let contract_address = self.state.delegation_address.as_ref().unwrap();
        let admin_address = bech32::decode("");

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
        let contract_address = self.state.delegation_address.as_ref().unwrap();
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
        let delegation_address = self.state.delegation_address.as_ref().unwrap();

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
        let delegation_address = self.state.delegation_address.as_ref().unwrap();

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
        let delegation_address = self.state.delegation_address.as_ref().unwrap();

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

    pub async fn set_state_active(&mut self, is_chain_simulator: bool) {
        match is_chain_simulator {
            true => {
                self.chain_simulator
                    .tx()
                    .from(&self.chain_sim_wallet_address)
                    .to(self.state.current_address())
                    .gas(30_000_000u64)
                    .typed(proxy::LiquidStakingProxy)
                    .set_state_active()
                    .returns(ReturnsResultUnmanaged)
                    .run()
                    .await
            }
            false => {
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
        };
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

    pub async fn delegate_tokens(&mut self) {
        let egld_amount = 1_000_000_000_000_000_000u64;

        let result_value = self
            .interactor
            .tx()
            .from(&self.chain_sim_wallet_address)
            .to(self.state.delegation_address())
            .gas(30_000_000u64)
            .typed(delegation_proxy::DelegationMockProxy)
            .delegate()
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn undelegate_tokens(&mut self) {
        let result_value = self
            .chain_simulator
            .tx()
            .from(&self.chain_sim_wallet_address)
            .to(self.state.delegation_address())
            .gas(30_000_000u64)
            .typed(delegation_proxy::DelegationMockProxy)
            .undelegate(1_000_000_000_000_000_000u64)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
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
        let contract_address = self.state.delegation_address.as_ref().unwrap();

        self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .delegation_contract_data(contract_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn get_account_storage(&mut self) {
        let result_value = self
            .chain_simulator
            .proxy
            .get_account_esdt_tokens(&self.chain_sim_wallet_address)
            .await;

        println!("Result: {result_value:?}");
    }
}

#[tokio::test]
#[cfg_attr(feature = "chain-simulator-tests", ignore)]
async fn test_setup_devnet() {
    let mut interact = ContractInteract::new().await;
    interact.deploy(false).await;
    interact.deploy_delegation_contract(false).await;
    interact.whitelist_delegation_contract(false).await;
    interact.set_state_active(false).await;
    interact.register_ls_token(false).await;
    interact.register_unstake_token(false).await;
    interact.add_liquidity(false).await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_setup_chain_simulator() {
    let mut interact = ContractInteract::new().await;
    interact.deploy(true).await;
    interact.deploy_delegation_contract(true).await;
    interact.whitelist_delegation_contract(true).await;
    interact.set_state_active(true).await;
    interact.register_ls_token(true).await;
    interact.register_unstake_token(true).await;
    interact.add_liquidity(true).await;
    interact.add_liquidity(true).await;
    interact.add_liquidity(true).await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_remove_liquidity() {
    let mut interact = ContractInteract::new().await;
    interact.remove_liquidity(true).await;
    interact.get_account_storage().await;
    interact.delegate_tokens().await;
    interact.undelegate_tokens().await;
    interact.get_delegation_contract_unbonded_amount().await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn unbound_tokens() {
    let mut interact = ContractInteract::new().await;
    interact
        .interactor
        .proxy
        .generate_blocks_until_epoch(14)
        .await
        .unwrap();
    interact.unbond_tokens(true).await;
}

#[tokio::test]
#[cfg_attr(feature = "chain-simulator-tests", ignore)]
async fn remove_liquidity_and_unbond_early() {
    let mut interact = ContractInteract::new().await;
    interact.remove_liquidity(false).await;
    interact
        .unbond_tokens_error(false, ExpectError(4, "The unstake period has not passed"))
        .await;
}

#[tokio::test]
#[cfg_attr(feature = "chain-simulator-tests", ignore)]
async fn test_views() {
    let mut interact = ContractInteract::new().await;
    interact.ls_token_supply().await;
    interact.virtual_egld_reserve().await;
}
