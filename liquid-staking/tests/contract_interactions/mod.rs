use crate::contract_setup::LiquidStakingContractSetup;
use elrond_wasm::types::{Address, BigUint};
use elrond_wasm_debug::{managed_address, num_bigint, rust_biguint, DebugApi};
use liquid_staking::config::{ConfigModule, UnstakeTokenAttributes};
use liquid_staking::LiquidStaking;

use delegation_mock::*;
use liquid_staking::delegation::DelegationModule;

impl<LiquidStakingContractObjBuilder> LiquidStakingContractSetup<LiquidStakingContractObjBuilder>
where
    LiquidStakingContractObjBuilder: 'static + Copy + Fn() -> liquid_staking::ContractObj<DebugApi>,
{
    pub fn deploy_staking_contract(
        &mut self,
        owner_address: &Address,
        egld_balance: u64,
        total_staked: u64,
        delegation_contract_cap: u64,
        nr_nodes: u64,
        apy: u64,
    ) {
        let rust_zero = rust_biguint!(0u64);
        let egld_balance_biguint = &&Self::exp18(egld_balance);
        let total_staked_biguint = BigUint::from(total_staked);

        self.b_mock
            .set_egld_balance(owner_address, egld_balance_biguint);

        let delegation_wrapper = self.b_mock.create_sc_account(
            &rust_zero,
            Some(&owner_address),
            delegation_mock::contract_obj,
            "delegation-mock.wasm",
        );

        self.b_mock
            .execute_tx(&owner_address, &delegation_wrapper, &rust_zero, |sc| {
                sc.init();
            })
            .assert_ok();

        self.b_mock
            .execute_tx(
                owner_address,
                &delegation_wrapper,
                egld_balance_biguint,
                |sc| {
                    sc.deposit_egld();
                },
            )
            .assert_ok();

        self.b_mock
            .execute_tx(owner_address, &self.sc_wrapper, &rust_zero, |sc| {
                sc.whitelist_delegation_contract(
                    managed_address!(delegation_wrapper.address_ref()),
                    total_staked_biguint,
                    delegation_contract_cap,
                    nr_nodes,
                    apy,
                );
            })
            .assert_ok();
    }

    pub fn add_liquidity(&mut self, caller: &Address, payment_amount: u64) {
        self.b_mock
            .execute_tx(
                caller,
                &self.sc_wrapper,
                &&Self::exp18(payment_amount),
                |sc| {
                    sc.add_liquidity();
                },
            )
            .assert_ok();
    }

    pub fn remove_liquidity(
        &mut self,
        caller: &Address,
        payment_token: &[u8],
        payment_amount: u64,
    ) {
        self.b_mock
            .execute_esdt_transfer(
                caller,
                &self.sc_wrapper,
                payment_token,
                0,
                &&Self::exp18(payment_amount),
                |sc| {
                    sc.remove_liquidity();
                },
            )
            .assert_ok();
    }

    pub fn claim_rewards(&mut self, caller: &Address) {
        let rust_zero = rust_biguint!(0u64);
        self.b_mock
            .execute_tx(caller, &self.sc_wrapper, &rust_zero, |sc| {
                sc.claim_rewards();
            })
            .assert_ok();
    }

    pub fn delegate_rewards(&mut self, caller: &Address) {
        let rust_zero = rust_biguint!(0u64);
        self.b_mock
            .execute_tx(caller, &self.sc_wrapper, &rust_zero, |sc| {
                sc.delegate_rewards();
            })
            .assert_ok();
    }

    pub fn unbond_tokens(
        &mut self,
        caller: &Address,
        payment_token: &[u8],
        token_nonce: u64,
        payment_amount: u64,
    ) {
        self.b_mock
            .execute_esdt_transfer(
                caller,
                &self.sc_wrapper,
                payment_token,
                token_nonce,
                &&Self::exp18(payment_amount),
                |sc| {
                    sc.unbond_tokens();
                },
            )
            .assert_ok();
    }

    pub fn setup_new_user(&mut self, egld_token_amount: u64) -> Address {
        let rust_zero = rust_biguint!(0);

        let new_user = self.b_mock.create_user_account(&rust_zero);
        self.b_mock
            .set_egld_balance(&new_user, &&Self::exp18(egld_token_amount));
        new_user
    }

    pub fn check_user_balance(&self, address: &Address, token_id: &[u8], token_balance: u64) {
        self.b_mock
            .check_esdt_balance(&address, token_id, &&Self::exp18(token_balance));
    }

    pub fn check_user_egld_balance(&self, address: &Address, token_balance: u64) {
        self.b_mock
            .check_egld_balance(&address, &&Self::exp18(token_balance));
    }

    pub fn check_contract_storage(
        &mut self,
        ls_token_supply: u64,
        virtual_egld_reserve: u64,
        rewards_reserve: u64,
        withdrawn_egld: u64,
        unstake_token_supply: u64,
    ) {
        self.b_mock
            .execute_query(&self.sc_wrapper, |sc| {
                assert_eq!(
                    sc.ls_token_supply().get(),
                    Self::to_managed_biguint(Self::exp18(ls_token_supply))
                );
                assert_eq!(
                    sc.virtual_egld_reserve().get(),
                    Self::to_managed_biguint(Self::exp18(virtual_egld_reserve))
                );
                assert_eq!(
                    sc.rewards_reserve().get(),
                    Self::to_managed_biguint(Self::exp18(rewards_reserve))
                );
                assert_eq!(
                    sc.withdrawn_egld().get(),
                    Self::to_managed_biguint(Self::exp18(withdrawn_egld))
                );
                assert_eq!(
                    sc.unstake_token_supply().get(),
                    Self::to_managed_biguint(Self::exp18(unstake_token_supply))
                );
            })
            .assert_ok();
    }

    // pub fn check_user_balance_denominated(
    //     &self,
    //     address: &Address,
    //     token_id: &[u8],
    //     token_balance: num_bigint::BigUint,
    // ) {
    //     self.b_mock
    //         .check_esdt_balance(&address, token_id, &token_balance);
    // }

    pub fn check_user_nft_balance(
        &self,
        address: &Address,
        token_id: &[u8],
        token_nonce: u64,
        token_balance: u64,
    ) {
        self.b_mock
            .check_nft_balance::<UnstakeTokenAttributes<DebugApi>>(
                &address,
                token_id,
                token_nonce,
                &&Self::exp18(token_balance),
                None,
            );
    }

    pub fn check_user_nft_balance_denominated(
        &self,
        address: &Address,
        token_id: &[u8],
        token_nonce: u64,
        token_balance: u64,
    ) {
        self.b_mock
            .check_nft_balance::<UnstakeTokenAttributes<DebugApi>>(
                &address,
                token_id,
                token_nonce,
                &num_bigint::BigUint::from(token_balance),
                None,
            );
    }
}
