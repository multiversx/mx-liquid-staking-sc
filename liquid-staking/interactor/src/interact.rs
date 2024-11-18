#![allow(non_snake_case)]

mod config;
mod proxy;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const STATE_FILE: &str = "state.toml";

pub async fn liquid_staking_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new().await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "upgrade" => interact.upgrade().await,
        "addLiquidity" => interact.add_liquidity().await,
        "removeLiquidity" => interact.remove_liquidity().await,
        "unbondTokens" => interact.unbond_tokens().await,
        "withdrawAll" => interact.withdraw_all().await,
        "claimRewards" => interact.claim_rewards().await,
        "recomputeTokenReserve" => interact.recompute_token_reserve().await,
        "delegateRewards" => interact.delegate_rewards().await,
        "getLsValueForPosition" => interact.get_ls_value_for_position().await,
        "registerLsToken" => interact.register_ls_token().await,
        "registerUnstakeToken" => interact.register_unstake_token().await,
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
        "getDelegationContractStakedAmount" => interact.get_delegation_contract_staked_amount().await,
        "getDelegationContractUnstakedAmount" => interact.get_delegation_contract_unstaked_amount().await,
        "getDelegationContractUnbondedAmount" => interact.get_delegation_contract_unbonded_amount().await,
        "setStateActive" => interact.set_state_active().await,
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
    contract_address: Option<Bech32Address>
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
    
        /// Returns the contract address
        pub fn current_address(&self) -> &Bech32Address {
            self.contract_address
                .as_ref()
                .expect("no known contract, deploy first")
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
    wallet_address: Address,
    contract_code: BytesValue,
    state: State
}

impl ContractInteract {
    pub async fn new() -> Self {
        let config = Config::new();
        let mut interactor = Interactor::new(config.gateway_uri())
            .await;

        interactor.set_current_dir_from_workspace("liquid-staking");
        let wallet_address = interactor.register_wallet(test_wallets::mike()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        // interactor.generate_blocks_until_epoch(1).await.unwrap();
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/liquid-staking.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state()
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));

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

    pub async fn add_liquidity(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .add_liquidity()
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn remove_liquidity(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .remove_liquidity()
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn unbond_tokens(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .unbond_tokens()
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn withdraw_all(&mut self) {
        let delegation_contract = bech32::decode("");

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
            .from(&self.wallet_address)
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

    pub async fn delegate_rewards(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
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

    pub async fn register_ls_token(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let token_display_name = ManagedBuffer::new_from_bytes(&b""[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b""[..]);
        let num_decimals = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .register_ls_token(token_display_name, token_ticker, num_decimals)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn register_unstake_token(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let token_display_name = ManagedBuffer::new_from_bytes(&b""[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b""[..]);
        let num_decimals = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .register_unstake_token(token_display_name, token_ticker, num_decimals)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn state(&mut self) {
        self
            .interactor
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
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let contract_address = bech32::decode("");
        let admin_address = bech32::decode("");
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
            .whitelist_delegation_contract(contract_address, admin_address, total_staked, delegation_contract_cap, nr_nodes, apy)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn change_delegation_contract_admin(&mut self) {
        let contract_address = bech32::decode("");
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
        let contract_address = bech32::decode("");
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
            .change_delegation_contract_params(contract_address, total_staked, delegation_contract_cap, nr_nodes, apy)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_delegation_status(&mut self) {
        self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .get_delegation_status()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn get_delegation_contract_staked_amount(&mut self) {
        let delegation_address = bech32::decode("");

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
        let delegation_address = bech32::decode("");

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
        let delegation_address = bech32::decode("");

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
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LiquidStakingProxy)
            .set_state_active()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
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

    pub async fn delegation_claim_status(&mut self) {
        self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .delegation_claim_status()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn delegation_contract_data(&mut self) {
        let contract_address = bech32::decode("");

        self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LiquidStakingProxy)
            .delegation_contract_data(contract_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

}

#[tokio::test]
async fn test_deploy() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
}
