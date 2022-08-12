#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use elrond_wasm::types::OperationCompletionStatus;
pub const DEFAULT_GAS_TO_CLAIM_REWARDS: u64 = 6_000_000;

pub mod config;
mod contexts;
pub mod delegation;
pub mod delegation_proxy;
pub mod errors;
mod events;
mod liquidity_pool;
mod ongoing_operation; // To import from wasm-modules after new release

use crate::{
    delegation::{ClaimStatus, ClaimStatusType},
    errors::*,
    ongoing_operation::{CONTINUE_OP, DEFAULT_MIN_GAS_TO_SAVE_PROGRESS, STOP_OP},
};

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
    + ongoing_operation::OngoingOperationModule
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

        self.delegation_proxy_obj()
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

        self.delegation_proxy_obj()
            .contract(delegation_contract.clone())
            .undelegate(egld_to_unstake.clone())
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
        self.delegation_proxy_obj()
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

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        let storage_cache = StorageCache::new(self);
        let delegation_addresses_mapper = self.delegation_addresses_list();
        let delegation_addresses_no = delegation_addresses_mapper.len();

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );

        let current_epoch = self.blockchain().get_block_epoch();
        let mut claim_status = self.load_operation::<ClaimStatus<Self::Api>>();

        self.can_proceed_claim_operation(&mut claim_status, current_epoch);

        let run_result = self.run_while_it_has_gas(DEFAULT_MIN_GAS_TO_SAVE_PROGRESS, || {
            if claim_status.current_iteration > delegation_addresses_no {
                return STOP_OP;
            }

            let delegation_address =
                delegation_addresses_mapper.get(claim_status.current_iteration);

            self.delegation_proxy_obj()
                .contract(delegation_address)
                .claim_rewards()
                .with_gas_limit(DEFAULT_GAS_TO_CLAIM_REWARDS)
                .transfer_execute();

            claim_status.current_iteration += 1;

            CONTINUE_OP
        });

        match run_result {
            OperationCompletionStatus::InterruptedBeforeOutOfGas => {
                self.save_progress(&claim_status);
            }
            OperationCompletionStatus::Completed => {
                claim_status.status = ClaimStatusType::Finished;
                self.delegation_claim_status().set(claim_status);
            }
        };
    }

    fn recompute_token_reserve(&self, claim_status: ClaimStatus<Self::Api>) {
        let withdrawed_egld_mapper = self.withdrawed_egld();
        let current_egld_balance = self.virtual_egld_reserve().get();
        let withdrawed_egld = withdrawed_egld_mapper.get();
        if current_egld_balance > &withdrawed_egld + &claim_status.starting_token_reserve
        {
            let rewards = current_egld_balance - withdrawed_egld - claim_status.starting_token_reserve;
            self.rewards_reserve().update(|new_rewards| *new_rewards += rewards);
            
        }
        withdrawed_egld_mapper.clear();
    }

    #[endpoint(delegateRewards)]
    fn delegate_rewards(&self) {
        let storage_cache = StorageCache::new(self);
        let claim_status = self.delegation_claim_status().get();
        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        require!(
            claim_status.status == ClaimStatusType::Finished,
            ERROR_CLAIM_REDELEGATE
        );

        let delegation_contract = self.get_next_delegation_contract();
        require!(!delegation_contract.is_zero(), ERROR_BAD_DELEGATION_ADDRESS);
        let rewards_reserve = self.rewards_reserve().get();

        self.delegation_proxy_obj()
            .contract(delegation_contract.clone())
            .delegate()
            .with_egld_transfer(rewards_reserve.clone())
            .async_call()
            .with_callback(self.callbacks().add_rewards_liquidity_callback(
                &delegation_contract,
                &rewards_reserve,
            ))
            .call_and_exit()

    }

    // views

    #[view]
    fn get_value_for_position(&self, ls_token_amount: BigUint) -> BigUint {
        let storage_cache = StorageCache::new(self);
        self.get_egld_amount(ls_token_amount, &storage_cache)
    }

    // proxy

    #[proxy]
    fn delegation_proxy_obj(&self) -> delegation_proxy::Proxy<Self::Api>;

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
                self.delegation_address(delegation_contract)
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

                self.emit_add_liquidity_event(&storage_cache, caller, user_payment.amount);
            }
            ManagedAsyncCallResult::Err(_) => {
                self.send().direct_egld(caller, &staked_tokens);
            }
        }
    }

    #[callback]
    fn add_rewards_liquidity_callback(
        &self,
        delegation_contract: &ManagedAddress,
        staked_tokens: &BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let mut storage_cache = StorageCache::new(self);
                self.delegation_address(delegation_contract)
                    .update(|contract_data| {
                        contract_data.total_staked_from_ls_contract += staked_tokens;
                    });

                self.rewards_reserve().clear();
                self.pool_add_liquidity(&staked_tokens, &mut storage_cache);
                let sc_address = self.blockchain().get_sc_address();
                self.emit_add_liquidity_event(&storage_cache, &sc_address,  BigUint::zero());
            }
            ManagedAsyncCallResult::Err(_) => {
                // TBD
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

                self.delegation_address(delegation_contract)
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
                self.delegation_address(delegation_contract)
                    .update(|contract_data| {
                        contract_data.total_undelegated_from_ls_contract -=
                            unbond_token_amount.clone();
                    });
                    self.withdrawed_egld()
                .update(|withdrawed_egld| *withdrawed_egld += &unbond_token_amount.clone());
                    
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
