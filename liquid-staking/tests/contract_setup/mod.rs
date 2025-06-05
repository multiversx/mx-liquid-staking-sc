use std::ops::Mul;

use liquid_staking::{setup::config::ConfigModule, LiquidStaking};
use multiversx_sc::types::{Address, BigUint, EsdtLocalRole};

use multiversx_sc_scenario::{
    imports::{BlockchainStateWrapper, ContractObjWrapper},
    managed_token_id, num_bigint, rust_biguint, DebugApi,
};

pub const LIQUID_STAKING_WASM_PATH: &str = "liquid-staking/output/liquid-staking.wasm";

pub static LS_TOKEN_ID: &[u8] = b"LSTOKEN-123456";
pub static UNSTAKE_TOKEN_ID: &[u8] = b"UNSTAKE-123456";

pub static ESDT_ROLES: &[EsdtLocalRole] = &[
    EsdtLocalRole::Mint,
    EsdtLocalRole::Burn,
    EsdtLocalRole::Transfer,
];

pub static SFT_ROLES: &[EsdtLocalRole] = &[
    EsdtLocalRole::NftCreate,
    EsdtLocalRole::NftAddQuantity,
    EsdtLocalRole::NftBurn,
];

pub struct LiquidStakingContractSetup<LiquidStakingContractObjBuilder>
where
    LiquidStakingContractObjBuilder: 'static + Copy + Fn() -> liquid_staking::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub sc_wrapper:
        ContractObjWrapper<liquid_staking::ContractObj<DebugApi>, LiquidStakingContractObjBuilder>,
}

impl<LiquidStakingContractObjBuilder> LiquidStakingContractSetup<LiquidStakingContractObjBuilder>
where
    LiquidStakingContractObjBuilder: 'static + Copy + Fn() -> liquid_staking::ContractObj<DebugApi>,
{
    pub fn new(sc_builder: LiquidStakingContractObjBuilder) -> Self {
        let rust_zero = rust_biguint!(0u64);
        let mut b_mock = BlockchainStateWrapper::new();
        let owner_address = b_mock.create_user_account(&rust_zero);

        let sc_wrapper = b_mock.create_sc_account(
            &rust_zero,
            Some(&owner_address),
            sc_builder,
            LIQUID_STAKING_WASM_PATH,
        );

        b_mock
            .execute_tx(&owner_address, &sc_wrapper, &rust_zero, |sc| {
                sc.init();
            })
            .assert_ok();

        b_mock
            .execute_tx(&owner_address, &sc_wrapper, &rust_zero, |sc| {
                sc.ls_token().set_token_id(managed_token_id!(LS_TOKEN_ID));
            })
            .assert_ok();

        b_mock
            .execute_tx(&owner_address, &sc_wrapper, &rust_zero, |sc| {
                sc.unstake_token()
                    .set_token_id(managed_token_id!(UNSTAKE_TOKEN_ID));
            })
            .assert_ok();

        b_mock.set_esdt_local_roles(sc_wrapper.address_ref(), LS_TOKEN_ID, ESDT_ROLES);
        b_mock.set_esdt_local_roles(sc_wrapper.address_ref(), UNSTAKE_TOKEN_ID, SFT_ROLES);

        LiquidStakingContractSetup {
            b_mock,
            owner_address,
            sc_wrapper,
        }
    }

    pub fn to_managed_biguint(value: num_bigint::BigUint) -> BigUint<DebugApi> {
        BigUint::from_bytes_be(&value.to_bytes_be())
    }

    pub fn exp18(value: u64) -> num_bigint::BigUint {
        value.mul(rust_biguint!(10).pow(18))
    }
}
