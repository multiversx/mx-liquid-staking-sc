#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use multiversx_sc::types::OperationCompletionStatus;
use multiversx_sc_modules::ongoing_operation::{
    CONTINUE_OP, DEFAULT_MIN_GAS_TO_SAVE_PROGRESS, STOP_OP,
};
pub const DEFAULT_GAS_TO_CLAIM_REWARDS: u64 = 6_000_000;
pub const MIN_GAS_FOR_ASYNC_CALL: u64 = 12_000_000;
pub const MIN_GAS_FOR_CALLBACK: u64 = 12_000_000;
pub const MIN_EGLD_TO_DELEGATE: u64 = 1_000_000_000_000_000_000;
pub const RECOMPUTE_BLOCK_OFFSET: u64 = 10;

pub mod config;
mod contexts;
pub mod delegation;
pub mod delegation_proxy;
pub mod errors;
mod events;
mod liquidity_pool;

use crate::{
    delegation::{ClaimStatus, ClaimStatusType},
    errors::*,
};

use config::{UnstakeTokenAttributes, UNBOND_PERIOD};
use contexts::base::*;
use liquidity_pool::State;

#[multiversx_sc::contract]
pub trait LiquidStaking<ContractReader>:
    liquidity_pool::LiquidityPoolModule
    + config::ConfigModule
    + events::EventsModule
    + delegation::DelegationModule
    + multiversx_sc_modules::ongoing_operation::OngoingOperationModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[init]
    fn init(&self) {
        self.state().set(State::Inactive);
        let current_epoch = self.blockchain().get_block_epoch();
        let current_round = self.blockchain().get_block_round();
        let claim_status = ClaimStatus {
            status: ClaimStatusType::Insufficient,
            last_claim_epoch: current_epoch,
            last_claim_block: current_round,
            ..Default::default()
        };

        self.delegation_claim_status().set_if_empty(claim_status);
    }

    #[payable("EGLD")]
    #[endpoint(addLiquidity)]
    fn add_liquidity(&self) {
        self.blockchain().check_caller_is_user_account();
        let storage_cache = StorageCache::new(self);
        let caller = self.blockchain().get_caller();

        let payment = self.call_value().egld_value().clone_value();
        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        require!(payment >= MIN_EGLD_TO_DELEGATE, ERROR_BAD_PAYMENT_AMOUNT);

        let delegation_contract = self.get_delegation_contract_for_delegate(&payment);
        let gas_for_async_call = self.get_gas_for_async_call();

        drop(storage_cache);
        self.delegation_proxy_obj()
            .contract(delegation_contract.clone())
            .delegate()
            .with_gas_limit(gas_for_async_call)
            .with_egld_transfer(payment.clone())
            .async_call()
            .with_callback(LiquidStaking::callbacks(self).add_liquidity_callback(
                caller,
                delegation_contract,
                payment,
            ))
            .call_and_exit()
    }

    #[callback]
    fn add_liquidity_callback(
        &self,
        caller: ManagedAddress,
        delegation_contract: ManagedAddress,
        staked_tokens: BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let mut storage_cache = StorageCache::new(self);
                self.delegation_contract_data(&delegation_contract)
                    .update(|contract_data| {
                        contract_data.total_staked_from_ls_contract += &staked_tokens;
                    });

                let ls_token_amount = self.pool_add_liquidity(&staked_tokens, &mut storage_cache);
                let user_payment = self.mint_ls_token(ls_token_amount);
                self.send().direct_esdt(
                    &caller,
                    &user_payment.token_identifier,
                    user_payment.token_nonce,
                    &user_payment.amount,
                );

                self.emit_add_liquidity_event(&storage_cache, &caller, user_payment.amount);
            }
            ManagedAsyncCallResult::Err(_) => {
                self.send().direct_egld(&caller, &staked_tokens);
                self.move_delegation_contract_to_back(delegation_contract);
            }
        }
    }

    #[payable("*")]
    #[endpoint(removeLiquidity)]
    fn remove_liquidity(&self) {
        self.blockchain().check_caller_is_user_account();
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
            payment.token_identifier == storage_cache.ls_token_id,
            ERROR_BAD_PAYMENT_TOKEN
        );
        require!(payment.amount > 0, ERROR_BAD_PAYMENT_AMOUNT);

        let egld_to_unstake = self.pool_remove_liquidity(&payment.amount, &mut storage_cache);
        require!(
            egld_to_unstake >= MIN_EGLD_TO_DELEGATE,
            ERROR_INSUFFICIENT_UNSTAKE_AMOUNT
        );
        self.burn_ls_token(&payment.amount);

        let delegation_contract = self.get_delegation_contract_for_undelegate(&egld_to_unstake);
        let gas_for_async_call = self.get_gas_for_async_call();

        drop(storage_cache);
        self.delegation_proxy_obj()
            .contract(delegation_contract.clone())
            .undelegate(egld_to_unstake.clone())
            .with_gas_limit(gas_for_async_call)
            .async_call()
            .with_callback(LiquidStaking::callbacks(self).remove_liquidity_callback(
                caller,
                delegation_contract,
                egld_to_unstake,
                payment.amount,
            ))
            .call_and_exit()
    }

    #[callback]
    fn remove_liquidity_callback(
        &self,
        caller: ManagedAddress,
        delegation_contract: ManagedAddress,
        egld_to_unstake: BigUint,
        ls_tokens_to_be_burned: BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let mut storage_cache = StorageCache::new(self);
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let current_epoch = self.blockchain().get_block_epoch();
                let unbond_epoch = current_epoch + UNBOND_PERIOD;

                self.delegation_contract_data(&delegation_contract)
                    .update(|contract_data| {
                        contract_data.total_staked_from_ls_contract -= &egld_to_unstake;
                        contract_data.total_unstaked_from_ls_contract += &egld_to_unstake;
                    });
                self.unstake_token_supply()
                    .update(|x| *x += &egld_to_unstake);

                let virtual_position = UnstakeTokenAttributes {
                    delegation_contract,
                    unstake_epoch: current_epoch,
                    unstake_amount: egld_to_unstake,
                    unbond_epoch,
                };

                let user_payment = self.mint_unstake_tokens(&virtual_position);
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
                self.move_delegation_contract_to_back(delegation_contract);
            }
        }
    }

    #[payable("*")]
    #[endpoint(unbondTokens)]
    fn unbond_tokens(&self) {
        self.blockchain().check_caller_is_user_account();
        let mut storage_cache = StorageCache::new(self);
        let payment = self.call_value().single_esdt();
        let caller = self.blockchain().get_caller();

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );

        require!(
            payment.token_identifier == self.unstake_token().get_token_id(),
            ERROR_BAD_PAYMENT_TOKEN
        );
        require!(payment.amount > 0, ERROR_BAD_PAYMENT_AMOUNT);

        let mut total_unstake_amount = BigUint::zero();

        let unstake_token_attributes: UnstakeTokenAttributes<Self::Api> = self
            .unstake_token()
            .get_token_attributes(payment.token_nonce);

        let current_epoch = self.blockchain().get_block_epoch();
        require!(
            current_epoch >= unstake_token_attributes.unbond_epoch,
            ERROR_UNSTAKE_PERIOD_NOT_PASSED
        );

        let delegation_contract = unstake_token_attributes.delegation_contract.clone();
        let unstake_amount = unstake_token_attributes.unstake_amount.clone();
        let delegation_contract_mapper = self.delegation_contract_data(&delegation_contract);
        let delegation_contract_data = delegation_contract_mapper.get();

        if delegation_contract_data.total_unbonded_from_ls_contract >= unstake_amount {
            delegation_contract_mapper.update(|contract_data| {
                contract_data.total_unstaked_from_ls_contract -= &unstake_amount;
                contract_data.total_unbonded_from_ls_contract -= &unstake_amount
            });

            total_unstake_amount += unstake_amount;
            storage_cache.total_withdrawn_egld -= &total_unstake_amount;
            self.unstake_token_supply()
                .update(|x| *x -= &total_unstake_amount);
            self.burn_unstake_tokens(payment.token_nonce);
            self.send().direct_egld(&caller, &total_unstake_amount)
        }
    }

    #[endpoint(withdrawAll)]
    fn withdraw_all(&self, provider: ManagedAddress) {
        self.blockchain().check_caller_is_user_account();
        let storage_cache = StorageCache::new(self);

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );

        let gas_for_async_call = self.get_gas_for_async_call();

        drop(storage_cache);
        self.delegation_proxy_obj()
            .contract(provider.clone())
            .withdraw()
            .with_gas_limit(gas_for_async_call)
            .async_call()
            .with_callback(LiquidStaking::callbacks(self).withdraw_tokens_callback(provider))
            .call_and_exit();
    }

    #[callback]
    fn withdraw_tokens_callback(
        &self,
        provider: ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let withdraw_amount = self.call_value().egld_value().clone_value();
                let mut storage_cache = StorageCache::new(self);
                let delegation_contract_mapper = self.delegation_contract_data(&provider);
                if withdraw_amount > 0u64 {
                    delegation_contract_mapper.update(|contract_data| {
                        contract_data.total_unbonded_from_ls_contract += &withdraw_amount
                    });
                    storage_cache.total_withdrawn_egld += &withdraw_amount;
                }
            }
            ManagedAsyncCallResult::Err(_) => {}
        }
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        let storage_cache = StorageCache::new(self);

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );

        let delegation_addresses_mapper = self.delegation_addresses_list();
        require!(
            !delegation_addresses_mapper.is_empty(),
            ERROR_NO_DELEGATION_CONTRACTS
        );
        let claim_status_mapper = self.delegation_claim_status();
        let old_claim_status = claim_status_mapper.get();
        let current_epoch = self.blockchain().get_block_epoch();
        let mut current_claim_status = self.load_operation::<ClaimStatus<Self::Api>>();

        self.check_claim_operation(&current_claim_status, old_claim_status, current_epoch);
        self.prepare_claim_operation(&mut current_claim_status, current_epoch);
        let mut delegation_addresses = self.addresses_to_claim();

        let run_result = self.run_while_it_has_gas(DEFAULT_MIN_GAS_TO_SAVE_PROGRESS, || {
            let current_node = delegation_addresses.pop_back().unwrap();
            let address = current_node.clone().into_value();

            self.delegation_proxy_obj()
                .contract(address)
                .claim_rewards()
                .with_gas_limit(DEFAULT_GAS_TO_CLAIM_REWARDS)
                .transfer_execute();

            if delegation_addresses.is_empty() {
                claim_status_mapper.set(current_claim_status.clone());
                return STOP_OP;
            }

            delegation_addresses.remove_node(&current_node);
            CONTINUE_OP
        });

        match run_result {
            OperationCompletionStatus::InterruptedBeforeOutOfGas => {
                self.save_progress(&current_claim_status);
            }
            OperationCompletionStatus::Completed => {
                claim_status_mapper.update(|claim_status| {
                    claim_status.status = ClaimStatusType::Finished;
                    claim_status.last_claim_block = self.blockchain().get_block_nonce();
                    claim_status.last_claim_epoch = self.blockchain().get_block_epoch();
                });
            }
        };
    }

    #[endpoint(recomputeTokenReserve)]
    fn recompute_token_reserve(&self) {
        let mut storage_cache = StorageCache::new(self);
        let claim_status_mapper = self.delegation_claim_status();
        let mut claim_status = claim_status_mapper.get();

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        require!(
            claim_status.status == ClaimStatusType::Finished,
            ERROR_RECOMPUTE_RESERVES
        );

        let current_block = self.blockchain().get_block_nonce();
        require!(
            current_block >= claim_status.last_claim_block + RECOMPUTE_BLOCK_OFFSET,
            ERROR_RECOMPUTE_TOO_SOON
        );

        let current_egld_balance = self
            .blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0);
        if current_egld_balance
            > &storage_cache.total_withdrawn_egld + &claim_status.starting_token_reserve
        {
            let rewards = &current_egld_balance
                - &storage_cache.total_withdrawn_egld
                - &claim_status.starting_token_reserve;
            storage_cache.rewards_reserve += rewards;
        }

        if storage_cache.rewards_reserve >= MIN_EGLD_TO_DELEGATE {
            claim_status.status = ClaimStatusType::Delegable;
        } else {
            claim_status.status = ClaimStatusType::Insufficient;
        }

        claim_status_mapper.set(claim_status);
    }

    #[endpoint(delegateRewards)]
    fn delegate_rewards(&self) {
        let mut storage_cache = StorageCache::new(self);
        let claim_status = self.delegation_claim_status().get();
        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        require!(
            claim_status.status == ClaimStatusType::Delegable,
            ERROR_CLAIM_REDELEGATE
        );

        let rewards_reserve = storage_cache.rewards_reserve.clone();
        storage_cache.rewards_reserve = BigUint::zero();
        let delegation_contract = self.get_delegation_contract_for_delegate(&rewards_reserve);
        let gas_for_async_call = self.get_gas_for_async_call();

        drop(storage_cache);
        self.delegation_proxy_obj()
            .contract(delegation_contract.clone())
            .delegate()
            .with_gas_limit(gas_for_async_call)
            .with_egld_transfer(rewards_reserve.clone())
            .async_call()
            .with_callback(
                LiquidStaking::callbacks(self)
                    .delegate_rewards_callback(delegation_contract, rewards_reserve),
            )
            .call_and_exit()
    }

    #[callback]
    fn delegate_rewards_callback(
        &self,
        delegation_contract: ManagedAddress,
        staked_tokens: BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let mut storage_cache = StorageCache::new(self);
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.delegation_contract_data(&delegation_contract)
                    .update(|contract_data| {
                        contract_data.total_staked_from_ls_contract += &staked_tokens;
                    });

                self.delegation_claim_status()
                    .update(|claim_status| claim_status.status = ClaimStatusType::Redelegated);

                storage_cache.virtual_egld_reserve += &staked_tokens;
                let sc_address = self.blockchain().get_sc_address();
                self.emit_add_liquidity_event(&storage_cache, &sc_address, BigUint::zero());
            }
            ManagedAsyncCallResult::Err(_) => {
                storage_cache.rewards_reserve += staked_tokens;
                self.move_delegation_contract_to_back(delegation_contract);
            }
        }
    }

    fn get_gas_for_async_call(&self) -> u64 {
        let gas_left = self.blockchain().get_gas_left();
        require!(
            gas_left > MIN_GAS_FOR_ASYNC_CALL + MIN_GAS_FOR_CALLBACK,
            ERROR_INSUFFICIENT_GAS
        );
        gas_left - MIN_GAS_FOR_CALLBACK
    }

    // views
    #[view(getLsValueForPosition)]
    fn get_ls_value_for_position(&self, ls_token_amount: BigUint) -> BigUint {
        let storage_cache = StorageCache::new(self);
        self.get_egld_amount(&ls_token_amount, &storage_cache)
    }

    // proxy

    #[proxy]
    fn delegation_proxy_obj(&self) -> delegation_proxy::Proxy<Self::Api>;
}
