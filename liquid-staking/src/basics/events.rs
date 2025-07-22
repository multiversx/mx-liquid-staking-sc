multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{basics::constants::Timestamp, contexts::base::StorageCache};

#[type_abi]
#[derive(TopEncode)]
pub struct AddLiquidityEvent<M: ManagedTypeApi> {
    caller: ManagedAddress<M>,
    ls_token_id: TokenIdentifier<M>,
    ls_token_amount: BigUint<M>,
    ls_token_supply: BigUint<M>,
    virtual_egld_reserve: BigUint<M>,
    rewards_reserve: BigUint<M>,
    block: u64,
    epoch: u64,
    timestamp: u64,
}

#[type_abi]
#[derive(TopEncode)]
pub struct RemoveLiquidityEvent<M: ManagedTypeApi> {
    caller: ManagedAddress<M>,
    ls_token_id: TokenIdentifier<M>,
    ls_token_amount: BigUint<M>,
    unstake_token_id: TokenIdentifier<M>,
    unstake_token_amount: BigUint<M>,
    ls_token_supply: BigUint<M>,
    virtual_egld_reserve: BigUint<M>,
    rewards_reserve: BigUint<M>,
    block: u64,
    epoch: u64,
    timestamp: u64,
}

#[multiversx_sc::module]
pub trait EventsModule:
    crate::setup::config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    fn emit_add_liquidity_event(
        &self,
        storage_cache: &StorageCache<Self>,
        caller: &ManagedAddress,
        ls_token_amount: BigUint,
    ) {
        let epoch = self.blockchain().get_block_epoch();
        self.add_liquidity_event(
            &storage_cache.ls_token_id,
            caller,
            epoch,
            &AddLiquidityEvent {
                caller: caller.clone(),
                ls_token_id: storage_cache.ls_token_id.clone(),
                ls_token_amount,
                ls_token_supply: storage_cache.ls_token_supply.clone(),
                virtual_egld_reserve: storage_cache.virtual_egld_reserve.clone(),
                rewards_reserve: self.rewards_reserve().get(),
                block: self.blockchain().get_block_nonce(),
                epoch,
                timestamp: self.blockchain().get_block_timestamp(),
            },
        )
    }

    fn emit_remove_liquidity_event(
        &self,
        storage_cache: &StorageCache<Self>,
        ls_token_amount: BigUint,
        unstake_token_amount: BigUint,
    ) {
        let epoch = self.blockchain().get_block_epoch();
        let caller = self.blockchain().get_caller();
        self.remove_liquidity_event(
            &storage_cache.ls_token_id,
            &caller,
            epoch,
            &RemoveLiquidityEvent {
                caller: caller.clone(),
                ls_token_id: storage_cache.ls_token_id.clone(),
                ls_token_amount,
                unstake_token_id: self.unstake_token().get_token_id(),
                unstake_token_amount,
                ls_token_supply: storage_cache.ls_token_supply.clone(),
                virtual_egld_reserve: storage_cache.virtual_egld_reserve.clone(),
                rewards_reserve: self.rewards_reserve().get(),
                block: self.blockchain().get_block_nonce(),
                epoch,
                timestamp: self.blockchain().get_block_timestamp(),
            },
        )
    }

    #[event("add_liquidity")]
    fn add_liquidity_event(
        &self,
        #[indexed] ls_token: &TokenIdentifier,
        #[indexed] caller: &ManagedAddress,
        #[indexed] epoch: u64,
        add_liquidity_event: &AddLiquidityEvent<Self::Api>,
    );

    #[event("remove_liquidity")]
    fn remove_liquidity_event(
        &self,
        #[indexed] ls_token: &TokenIdentifier,
        #[indexed] caller: &ManagedAddress,
        #[indexed] epoch: u64,
        remove_liquidity_event: &RemoveLiquidityEvent<Self::Api>,
    );

    #[event("successful_claim")]
    fn successful_claim_event(
        &self,
        amount_available_to_claim: BigUint,
        #[indexed] caller: &ManagedAddress,
    );

    #[event("failed_claim")]
    fn failed_claim_event(&self, #[indexed] caller: &ManagedAddress);

    #[event("tokens_locked_for_delegate_vote_event")]
    fn tokens_locked_for_delegate_vote_event(
        &self,
        #[indexed] voter: &ManagedAddress,
        amount: &BigUint,
        #[indexed] lock_until: Timestamp,
    );
}
