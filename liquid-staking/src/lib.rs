#![no_std]
#![allow(clippy::vec_init_then_push)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod config;
mod contexts;
pub mod delegation;
pub mod delegation_proxy;
pub mod errors;
mod events;
mod liquidity_pool;

use crate::errors::*;

use config::{UnstakeTokenAttributes, UNBOND_PERIOD};
use contexts::base::*;
use liquidity_pool::State;

pub type AddLiquidityResultType<BigUint> =
    MultiValue3<EsdtTokenPayment<BigUint>, EsdtTokenPayment<BigUint>, EsdtTokenPayment<BigUint>>;

pub type RemoveLiquidityResultType<BigUint> =
    MultiValue2<EsdtTokenPayment<BigUint>, EsdtTokenPayment<BigUint>>;

pub type SwapTokensFixedInputResultType<BigUint> = EsdtTokenPayment<BigUint>;

pub type SwapTokensFixedOutputResultType<BigUint> =
    MultiValue2<EsdtTokenPayment<BigUint>, EsdtTokenPayment<BigUint>>;

#[elrond_wasm::contract]
pub trait Pair<ContractReader>:
    liquidity_pool::LiquidityPoolModule
    + config::ConfigModule
    + events::EventsModule
    + delegation::DelegationModule
{
    #[init]
    fn init(&self) {
        self.state().set(State::Inactive);
    }

    // TODO - check if add_initial_liquidity is necessary
    #[payable("EGLD")]
    #[endpoint(addLiquidity)]
    fn add_liquidity(&self) {
        let storage_cache = StorageCache::new(self);
        let caller = self.blockchain().get_caller();

        let payment = self.call_value().egld_value();
        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        require!(payment > 0, ERROR_BAD_PAYMENT_AMOUNT);

        let delegation_contract = self.get_next_delegation_contract();
        require!(!delegation_contract.is_zero(), ERROR_BAD_DELEGATION_ADDRESS);

        self.delegation_sc_proxy_obj()
            .contract(delegation_contract.clone())
            .delegate()
            .with_egld_transfer(payment.clone())
            .async_call()
            .with_callback(self.callbacks().add_liquidity_callback(
                &caller,
                &delegation_contract,
                &payment,
            ))
            .call_and_exit()
    }

    #[payable("*")]
    #[endpoint(removeLiquidity)]
    fn remove_liquidity(&self) -> RemoveLiquidityResultType<Self::Api> {
        let mut storage_cache = StorageCache::new(self);
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        require!(
            storage_cache.ls_token_id.is_valid_esdt_identifier(),
            ERROR_LS_TOKEN_NOT_ISSUED
        );
        require!(
            payment.token_identifier == storage_cache.ls_token_id && payment.amount > 0,
            ERROR_BAD_PAYMENT_TOKENS
        );

        let egld_to_unstake = self.pool_remove_liquidity(&payment.amount, &mut storage_cache);
        self.burn_ls_token(&payment.amount);

        let delegation_contract = self.get_next_delegation_contract();
        require!(!delegation_contract.is_zero(), ERROR_BAD_DELEGATION_ADDRESS);

        self.delegation_sc_proxy_obj()
            .contract(delegation_contract.clone())
            .unDelegate(egld_to_unstake.clone())
            .async_call()
            .with_callback(self.callbacks().remove_liquidity_callback(
                &caller,
                &delegation_contract,
                egld_to_unstake,
                payment.amount,
            ))
            .call_and_exit()
    }

    #[payable("*")]
    #[endpoint(unbondTokens)]
    fn unbond_tokens(&self) {
        let storage_cache = StorageCache::new(self);
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        require!(
            payment.token_identifier == self.unstake_token().get_token_id() && payment.amount > 0,
            ERROR_BAD_PAYMENT_TOKENS
        );

        let unstake_token_attributes: UnstakeTokenAttributes<Self::Api> = self
            .unstake_token()
            .get_token_attributes(payment.token_nonce);

        let current_epoch = self.blockchain().get_block_epoch();
        require!(
            current_epoch >= unstake_token_attributes.unbond_epoch,
            ERROR_UNSTAKE_PERIOD_NOT_PASSED
        );

        let delegation_contract = unstake_token_attributes.delegation_contract;
        self.delegation_sc_proxy_obj()
            .contract(delegation_contract.clone())
            .withdraw()
            .async_call()
            .with_callback(self.callbacks().withdraw_tokens_callback(
                &caller,
                &delegation_contract,
                payment.token_nonce,
                payment.amount,
            ))
            .call_and_exit();
    }

    // views

    #[view]
    fn get_value_for_position(&self, ls_token_amount: BigUint) -> BigUint {
        let storage_cache = StorageCache::new(self);
        self.get_egld_amount(ls_token_amount, &storage_cache)
    }

    // proxy

    #[proxy]
    fn delegation_sc_proxy_obj(&self) -> delegation_proxy::Proxy<Self::Api>;

    // callbacks

    #[callback]
    fn add_liquidity_callback(
        &self,
        caller: &ManagedAddress,
        delegation_contract: &ManagedAddress,
        staked_tokens: &BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let mut storage_cache = StorageCache::new(self);
                self.delegation_sc_address(delegation_contract)
                    .update(|contract_data| {
                        contract_data.total_staked_from_ls_contract += staked_tokens;
                    });

                let ls_token_amount = self.pool_add_liquidity(&staked_tokens, &mut storage_cache);
                let user_payment = self.mint_ls_token(ls_token_amount);
                self.send().direct_esdt(
                    &caller,
                    &user_payment.token_identifier,
                    user_payment.token_nonce,
                    &user_payment.amount,
                );

                self.emit_add_liquidity_event(&storage_cache, user_payment.amount);
            }
            ManagedAsyncCallResult::Err(_) => {
                self.send().direct_egld(caller, &staked_tokens);
            }
        }
    }

    #[callback]
    fn remove_liquidity_callback(
        &self,
        caller: &ManagedAddress,
        delegation_contract: &ManagedAddress,
        egld_to_unstake: BigUint,
        ls_tokens_to_be_burned: BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let mut storage_cache = StorageCache::new(self);
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let current_epoch = self.blockchain().get_block_epoch();
                let unbond_epoch = current_epoch + UNBOND_PERIOD;

                let virtual_position = UnstakeTokenAttributes {
                    delegation_contract: delegation_contract.clone(),
                    unstake_epoch: current_epoch,
                    unbond_epoch: unbond_epoch,
                };

                self.delegation_sc_address(delegation_contract)
                    .update(|contract_data| {
                        contract_data.total_staked_from_ls_contract -= egld_to_unstake.clone();
                        contract_data.total_undelegated_from_ls_contract += egld_to_unstake.clone();
                    });

                let user_payment = self.mint_unstake_tokens(egld_to_unstake, &virtual_position);
                self.send().direct_esdt(
                    &caller,
                    &user_payment.token_identifier,
                    user_payment.token_nonce,
                    &user_payment.amount,
                );

                self.emit_remove_liquidity_event(
                    &storage_cache,
                    ls_tokens_to_be_burned,
                    user_payment.amount,
                );
            }
            ManagedAsyncCallResult::Err(_) => {
                let ls_token_amount = self.pool_add_liquidity(&egld_to_unstake, &mut storage_cache);

                let user_payment = self.mint_ls_token(ls_token_amount);

                self.send().direct_esdt(
                    &caller,
                    &user_payment.token_identifier,
                    user_payment.token_nonce,
                    &user_payment.amount,
                );
            }
        }
    }

    #[callback]
    fn withdraw_tokens_callback(
        &self,
        caller: &ManagedAddress,
        delegation_contract: &ManagedAddress,
        unbond_token_nonce: u64,
        unbond_token_amount: BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.delegation_sc_address(delegation_contract)
                    .update(|contract_data| {
                        contract_data.total_undelegated_from_ls_contract -=
                            unbond_token_amount.clone();
                    });
                self.burn_unstake_tokens(unbond_token_nonce, &unbond_token_amount);
                self.send().direct_egld(caller, &unbond_token_amount)
            }
            ManagedAsyncCallResult::Err(_) => {
                let unstake_token_id = self.unstake_token().get_token_id();
                self.send().direct_esdt(
                    caller,
                    &unstake_token_id,
                    unbond_token_nonce,
                    &unbond_token_amount,
                )
            }
        }
    }
}
