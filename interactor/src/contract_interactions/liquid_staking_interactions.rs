use crate::{liquid_staking_proxy, Interact};
use multiversx_sc_snippets::imports::*;

impl Interact {
    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .init()
            .code(&self.liquid_staking_contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;

        let new_address_bech32 = Bech32Address::from(&new_address);
        self.state.set_address(new_address_bech32.clone());

        let new_address_string = new_address_bech32.to_string();

        println!("new address: {new_address_string}");
    }

    pub async fn upgrade(&mut self) {
        let response = self
            .interactor
            .tx()
            .to(self.state.liquid_staking_address())
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .upgrade()
            .code(&self.liquid_staking_contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewAddress)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_liquidity(&mut self, caller: Bech32Address, liquidity: u128) {
        self.interactor
            .tx()
            .from(caller)
            .to(self.state.liquid_staking_address())
            .gas(50_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .add_liquidity()
            .egld(liquidity)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn remove_liquidity(&mut self, caller: Bech32Address, token_id: &str, amount: u128) {
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(amount);

        self.interactor
            .tx()
            .from(caller)
            .to(self.state.liquid_staking_address())
            .gas(50_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .remove_liquidity()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn unbond_tokens(&mut self, caller: Bech32Address, token_id: &str, amount: u128) {
        let token_nonce = 1u64;
        let token_amount = BigUint::<StaticApi>::from(amount);

        self.interactor
            .tx()
            .from(caller)
            .to(self.state.liquid_staking_address())
            .gas(50_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .unbond_tokens()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn withdraw_all(&mut self, caller: Bech32Address, error: Option<ExpectError<'_>>) {
        let delegation_contract = self.state.delegation_address();

        let tx = self
            .interactor
            .tx()
            .from(caller)
            .to(self.state.liquid_staking_address())
            .gas(50_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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

    pub async fn claim_rewards(&mut self, caller: Bech32Address, error: Option<ExpectError<'_>>) {
        let tx = self
            .interactor
            .tx()
            .from(caller)
            .to(self.state.liquid_staking_address())
            .gas(50_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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

    pub async fn recompute_token_reserve(&mut self, caller: Bech32Address) {
        let response = self
            .interactor
            .tx()
            .from(caller)
            .to(self.state.liquid_staking_address())
            .gas(30_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .recompute_token_reserve()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn delegate_rewards(
        &mut self,
        caller: Bech32Address,
        error: Option<ExpectError<'_>>,
    ) {
        let tx = self
            .interactor
            .tx()
            .from(caller)
            .to(self.state.liquid_staking_address())
            .gas(50_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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

    pub async fn get_ls_value_for_position(&mut self, ls_token_amount: u128) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .get_ls_value_for_position(ls_token_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn register_ls_token(
        &mut self,
        display_name: &str,
        ticker: &str,
        num_decimals: u32,
        amount: u128,
    ) -> String {
        let egld_amount = BigUint::<StaticApi>::from(amount);

        let token_display_name = ManagedBuffer::from(display_name);
        let token_ticker = ManagedBuffer::from(ticker);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.liquid_staking_address())
            .gas(90_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .register_ls_token(token_display_name, token_ticker, num_decimals)
            .egld(egld_amount)
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await
    }

    pub async fn register_unstake_token(
        &mut self,
        display_name: &str,
        ticker: &str,
        num_decimals: u32,
        amount: u128,
    ) -> String {
        let egld_amount = BigUint::<StaticApi>::from(amount);

        let token_display_name = ManagedBuffer::from(display_name);
        let token_ticker = ManagedBuffer::from(ticker);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.liquid_staking_address())
            .gas(90_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .register_unstake_token(token_display_name, token_ticker, num_decimals)
            .egld(egld_amount)
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await
    }

    pub async fn state(&mut self) {
        self.interactor
            .query()
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .state()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn ls_token(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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
            .to(self.state.liquid_staking_address())
            .gas(30_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .clear_ongoing_whitelist_op()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn whitelist_delegation_contract(
        &mut self,
        amount: u128,
        contract_address: Bech32Address,
        admin_address: Bech32Address,
        staked: u128,
        delegation_cap: u128,
        nr_nodes: u64,
        apy: u64,
    ) {
        let egld_amount = BigUint::<StaticApi>::from(amount);

        let total_staked = BigUint::<StaticApi>::from(staked);
        let delegation_contract_cap = BigUint::<StaticApi>::from(delegation_cap);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.liquid_staking_address())
            .gas(30_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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

    pub async fn change_delegation_contract_admin(
        &mut self,
        contract_address: Bech32Address,
        admin_address: Bech32Address,
    ) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.liquid_staking_address())
            .gas(30_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .change_delegation_contract_admin(contract_address, admin_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn change_delegation_contract_params(
        &mut self,
        contract_address: Bech32Address,
        staked: u128,
        delegation_cap: u128,
        nr_nodes: u64,
        apy: u64,
    ) {
        let total_staked = BigUint::<StaticApi>::from(staked);
        let delegation_contract_cap = BigUint::<StaticApi>::from(delegation_cap);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.liquid_staking_address())
            .gas(30_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .get_delegation_status()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn get_delegation_contract_staked_amount(
        &mut self,
        delegation_address: Bech32Address,
    ) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .get_delegation_contract_staked_amount(delegation_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_delegation_contract_unstaked_amount(
        &mut self,
        delegation_address: Bech32Address,
    ) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .get_delegation_contract_unstaked_amount(delegation_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_delegation_contract_unbonded_amount(
        &mut self,
        delegation_address: Bech32Address,
    ) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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
            .to(self.state.liquid_staking_address())
            .gas(30_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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
            .to(self.state.liquid_staking_address())
            .gas(30_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
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
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .addresses_to_claim()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn delegation_claim_status(&mut self) {
        self.interactor
            .query()
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .delegation_claim_status()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn delegation_contract_data(&mut self, delegation_address: Bech32Address) {
        self.interactor
            .query()
            .to(self.state.liquid_staking_address())
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .delegation_contract_data(delegation_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }
}
