// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct LiquidStakingProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for LiquidStakingProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = LiquidStakingProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        LiquidStakingProxyMethods { wrapped_tx: tx }
    }
}

pub struct LiquidStakingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> LiquidStakingProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> LiquidStakingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> LiquidStakingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn add_liquidity(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("addLiquidity")
            .original_result()
    }

    pub fn remove_liquidity(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("removeLiquidity")
            .original_result()
    }

    pub fn unbond_tokens(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("unbondTokens")
            .original_result()
    }

    pub fn withdraw_all<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        delegation_contract: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("withdrawAll")
            .argument(&delegation_contract)
            .original_result()
    }

    pub fn claim_rewards(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claimRewards")
            .original_result()
    }

    pub fn recompute_token_reserve(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("recomputeTokenReserve")
            .original_result()
    }

    pub fn delegate_rewards(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("delegateRewards")
            .original_result()
    }

    pub fn get_ls_value_for_position<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        ls_token_amount: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLsValueForPosition")
            .argument(&ls_token_amount)
            .original_result()
    }

    pub fn register_ls_token<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<usize>,
    >(
        self,
        token_display_name: Arg0,
        token_ticker: Arg1,
        num_decimals: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("registerLsToken")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .argument(&num_decimals)
            .original_result()
    }

    pub fn register_unstake_token<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<usize>,
    >(
        self,
        token_display_name: Arg0,
        token_ticker: Arg1,
        num_decimals: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("registerUnstakeToken")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .argument(&num_decimals)
            .original_result()
    }

    pub fn state(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, State> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getState")
            .original_result()
    }

    pub fn ls_token(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLsTokenId")
            .original_result()
    }

    pub fn ls_token_supply(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLsSupply")
            .original_result()
    }

    pub fn virtual_egld_reserve(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getVirtualEgldReserve")
            .original_result()
    }

    pub fn rewards_reserve(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRewardsReserve")
            .original_result()
    }

    pub fn unstake_token(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUnstakeTokenId")
            .original_result()
    }

    pub fn clear_ongoing_whitelist_op(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("clearOngoingWhitelistOp")
            .original_result()
    }

    pub fn whitelist_delegation_contract<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
        Arg4: ProxyArg<u64>,
        Arg5: ProxyArg<u64>,
    >(
        self,
        contract_address: Arg0,
        admin_address: Arg1,
        total_staked: Arg2,
        delegation_contract_cap: Arg3,
        nr_nodes: Arg4,
        apy: Arg5,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("whitelistDelegationContract")
            .argument(&contract_address)
            .argument(&admin_address)
            .argument(&total_staked)
            .argument(&delegation_contract_cap)
            .argument(&nr_nodes)
            .argument(&apy)
            .original_result()
    }

    pub fn change_delegation_contract_admin<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        contract_address: Arg0,
        admin_address: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeDelegationContractAdmin")
            .argument(&contract_address)
            .argument(&admin_address)
            .original_result()
    }

    pub fn change_delegation_contract_params<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
        Arg3: ProxyArg<u64>,
        Arg4: ProxyArg<u64>,
    >(
        self,
        contract_address: Arg0,
        total_staked: Arg1,
        delegation_contract_cap: Arg2,
        nr_nodes: Arg3,
        apy: Arg4,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeDelegationContractParams")
            .argument(&contract_address)
            .argument(&total_staked)
            .argument(&delegation_contract_cap)
            .argument(&nr_nodes)
            .argument(&apy)
            .original_result()
    }

    pub fn get_delegation_status(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ClaimStatusType> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDelegationStatus")
            .original_result()
    }

    pub fn get_delegation_contract_staked_amount<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        delegation_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDelegationContractStakedAmount")
            .argument(&delegation_address)
            .original_result()
    }

    pub fn get_delegation_contract_unstaked_amount<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        delegation_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDelegationContractUnstakedAmount")
            .argument(&delegation_address)
            .original_result()
    }

    pub fn get_delegation_contract_unbonded_amount<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        delegation_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDelegationContractUnbondedAmount")
            .argument(&delegation_address)
            .original_result()
    }

    pub fn set_state_active(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setStateActive")
            .original_result()
    }

    pub fn set_state_inactive(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setStateInactive")
            .original_result()
    }

    pub fn delegation_addresses_list(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDelegationAddressesList")
            .original_result()
    }

    pub fn addresses_to_claim(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAddressesToClaim")
            .original_result()
    }

    pub fn delegation_claim_status(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ClaimStatus> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDelegationClaimStatus")
            .original_result()
    }

    pub fn delegation_contract_data<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        contract_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, DelegationContractData<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDelegationContractData")
            .argument(&contract_address)
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Eq, Copy, Clone, Debug)]
pub enum State {
    Inactive,
    Active,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone)]
pub struct AddLiquidityEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub ls_token_id: TokenIdentifier<Api>,
    pub ls_token_amount: BigUint<Api>,
    pub ls_token_supply: BigUint<Api>,
    pub virtual_egld_reserve: BigUint<Api>,
    pub rewards_reserve: BigUint<Api>,
    pub block: u64,
    pub epoch: u64,
    pub timestamp: u64,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone)]
pub struct RemoveLiquidityEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub ls_token_id: TokenIdentifier<Api>,
    pub ls_token_amount: BigUint<Api>,
    pub unstake_token_id: TokenIdentifier<Api>,
    pub unstake_token_amount: BigUint<Api>,
    pub ls_token_supply: BigUint<Api>,
    pub virtual_egld_reserve: BigUint<Api>,
    pub rewards_reserve: BigUint<Api>,
    pub block: u64,
    pub epoch: u64,
    pub timestamp: u64,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone)]
pub enum ClaimStatusType {
    Finished,
    Delegable,
    Insufficient,
    Redelegated,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone)]
pub struct ClaimStatus {
    pub status: ClaimStatusType,
    pub last_claim_epoch: u64,
    pub last_claim_block: u64,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Eq, Debug)]
pub struct DelegationContractData<Api>
where
    Api: ManagedTypeApi,
{
    pub admin_address: ManagedAddress<Api>,
    pub total_staked: BigUint<Api>,
    pub delegation_contract_cap: BigUint<Api>,
    pub nr_nodes: u64,
    pub apy: u64,
    pub total_staked_from_ls_contract: BigUint<Api>,
    pub total_unstaked_from_ls_contract: BigUint<Api>,
    pub total_unbonded_from_ls_contract: BigUint<Api>,
    pub egld_in_ongoing_undelegation: BigUint<Api>,
}
