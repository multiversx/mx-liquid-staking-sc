use crate::contexts::base::StorageCache;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TypeAbi, TopEncode)]
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

#[derive(TypeAbi, TopEncode)]
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

#[elrond_wasm::module]
pub trait EventsModule:
    crate::config::ConfigModule
    + elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
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
                rewards_reserve: storage_cache.rewards_reserve.clone(),
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
                rewards_reserve: storage_cache.rewards_reserve.clone(),
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
}
