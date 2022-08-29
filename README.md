# Liquid Staking Smart Contract


## Abstract

Elrond Network is a proof-of-stake blockchain protocol that seeks to offer extremely fast transaction speeds by using adaptive state sharding. By being a proof-of-stake blockchain, it incentives holders to stake their assets (EGLD), in order to increase the network's security, by offering them staking rewards at a protocol level. As more and more products appear on the network, it raises the question for the holder, if he/she should just use their EGLD in the ecosystem, instead of just staking it. This Liquid Staking SC tries to solve this problem by offering users the ability to stake their EGLD in exchange for another token, that can be further implemented in various products within the Elrond ecosystem, while still receiving the staking rewards from the protocol.


## Introduction

The Liquid Staking Smart Contract allows users to stake their EGLD in return of lsEGLD, a fungible ESDT that can be used in multiple ways in the Elrond Network ecosystem, all while retaining the standard staking rewards. It offers users the possibility to stake or unstake their EGLD, and their position only continues to increase as rewards are cumulated and redelegated. This means that, in time, the value of lsEGLD will only continue to outgrow that of the EGLD, as rewards are compounded every epoch. Also, it offers delegation contract owners to maintain the status of their delegation contracts, through whitelisted admin-only endpoints.


## Endpoints

### init

```rust
    #[init]
    fn init(&self);
```

The init function is called when deploying/upgrading the smart contract. It sets various parameters that the contract needs, including setting the contract's state as Inactive.


### addLiquidity

```rust
    #[payable("EGLD")]
    #[endpoint(addLiquidity)]
    fn add_liquidity(&self);
```

The ```addLiquidity``` endpoint is the one that allows users to stake their __EGLD__ in exchange for __lsEGLD__. After the initial checks are verified, the endpoint chooses a delegation address from the available delegation contracts list, and tries to delegate those __EGLD__ tokens by sending an async call, hooked with ```add_liquidity_callback```, to that address.
In the callback, in case of a succesful result, the staking data for that delegation contract is updated accordingly, and liquidity is then computed and added, resulting in the total __lsEGLD__ that needs to be created. The __lsEGLD__ fungible ESDTs are then minted and sent to the initial caller. In case of an unsuccesful delegation, the __EGLD__ tokens are then sent back to the caller.
One important observation here is that in time, with each redelegation of rewards, the value of the __lsEGLD__ token will outgrow that of the __EGLD__ token, so users will receive less and less __lsEGLD__ tokens, in exchange for their __EGLD__.


### removeLiquidity

```rust
    #[payable("*")]
    #[endpoint(removeLiquidity)]
    fn remove_liquidity(&self);
```

This endpoint allows users to unstake their __EGLD__, by sending a payment of __lsEGLD__. Unlike the ```addLiquidity``` endpoint, the liquidity is first removed and the __lsEGLD__ token burnt, in order to get the correct amount of __EGLD__ that needs to be undelegated. Again, a new delegation contract with enough available staked tokens is chosen and then the ```undelegate``` function is called through an async call, hooked with the ```remove_liquidity_callback```.
In the callback, in case of a succesful undelegation, the supply storage is updated accordingly and then a new NFT is minted and sent to the initial caller, containing the necessary data for the later withdraw operation.

```rust
pub struct UnstakeTokenAttributes {
    pub delegation_contract: ManagedAddress,
    pub unstake_epoch: u64,
    pub unstake_amount: BigUint,
    pub unbond_epoch: u64,
}
```

In case of an unsuccesful undelegation, the contract adds back the liquidity, mints and then sends back the __lsEGLD__ token to the caller.


### unbondTokens

```rust
     #[payable("*")]
    #[endpoint(unbondTokens)]
    fn unbond_tokens(&self);
```

The ```unbondTokens``` endpoint allows users to receive back their EGLD after the unbond period has passed. It receives only the payment of __unstake_token_NFT__, where it first checks if the unbond period has passed, the delegation contract from where the tokens were unstaked and how much the users must receive. After that, it sends an async call to that contract, hooked with the ```withdraw_tokens_callback```.
In the callback, in case of a succesful result, the reserves storage is updated, the NFT is burnt and the __EGLD__ tokens are sent to the caller. In case of an unsuccesful result, the NFT is sent back to the user.


### claimRewards

```rust
    #[endpoint(claimRewards)]
    fn claim_rewards(&self);
```

The ```claimRewards``` endpoint is callable by any users, and allows each epoch to claim all pending rewards and store them in a __rewards_reserve__ storage, until those rewards are redelegated and are taken into account in the general __virtual_egld_reserve__ storage.
The ```claimRewards``` function implements an ongoing operation mechanism, that claims rewards from delegation contracts until the list is completely covered of until it runs out of gas, in which case it saves the current iteration in the delegation contract list and is able to continue from where it left off. After all delegation contracts have been covered, the token reserves are then updated, including the __rewards_reserve__ and the __withdrawn_egld__ variable, that takes into account any __EGLD__ that was withdrawn during the claiming operation.

The available claim status types are:

```rust
pub enum ClaimStatusType {
    None,
    Pending,
    Delegable,
    Insufficient,
    Redelegated,
}
```

The workflow is as follows:
- In order to start a new ```claimRewards``` operation, the previous claim status must be __Insufficient__ or __Redelegated__. Once the operation has started, the new claim operation is updated to the __Pending__ status. After the claim operation has finished, in case the total available rewards are greater than the minimum delegation amount required (__1 EGLD__), the status is then updated to __Delegable__, otherwise it is updated to __Insufficient__. The ```delegateRewards``` is then callable (only if the status is __Delegable__) then updates the status to __Redelegated__ which allows for the cycle to start once again.


### delegateRewards

```rust
    #[endpoint(delegateRewards)]
    fn delegate_rewards(&self);
```

As stated before, the ```delegateRewards``` endpoint allows for the delegation of all the cumulated rewards from the ```claimRewards``` endpoint, but only if the rewards amount is greater than the minimum amount required (marked by the __Delegable__ claim status). If all check conditions are met, then a new delegation contract is chosen from the whitelist and a new delegation operation is called through an async call hooked with the ```delegate_rewards_callback```.
In the callback, if the result is succesful, the reserves are updated accordingly, including the __virtual_egld_reserve__ and the __rewards_reserve__ which is set to 0.


### whitelistDelegationContract

```rust
    #[only_owner]
    #[endpoint(whitelistDelegationContract)]
    fn whitelist_delegation_contract(
        &self,
        contract_address: ManagedAddress,
        total_staked: BigUint,
        delegation_contract_cap: u64,
        nr_nodes: u64,
        apy: u64,
    );
```

Endpoint that allows the owner to whitelist a delegation contract with a set of parameters, sent as arguments (__DelegationContractData__). From this list, the first 4 variables are user updatable, while __total_staked_from_ls_contract__ and __total_undelegated_from_ls_contract__ variables are automatically updated throughout the contract's workflow.

```rust
pub struct DelegationContractData {
    pub total_staked: BigUint,
    pub delegation_contract_cap: u64,
    pub nr_nodes: u64,
    pub apy: u64,
    pub total_staked_from_ls_contract: BigUint,
    pub total_undelegated_from_ls_contract: BigUint,
}
```


### changeDelegationContractParams

```rust
    #[endpoint(changeDelegationContractParams)]
    fn change_delegation_contract_params(
        &self,
        contract_address: ManagedAddress,
        total_staked: BigUint,
        delegation_contract_cap: u64,
        nr_nodes: u64,
        apy: u64,
    );
```

Endpoint that allows the admin of a whitelisted delegation contract to update the given parameters, by sending them as arguments. The __total_staked_from_ls_contract__ and __total_undelegated_from_ls_contract__ variables remain unchanged.


### registerLsToken

```rust
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(registerLsToken)]
    fn register_ls_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    );  
```

A setup endpoint, that receives an exact payment of __0.05 EGLD__ (amount required for the deployment of a new token) and that issues and sets all roles for the newly created token. Being a fungible ESDT, the __lsEGLD__ is handled through a __FungibleTokenMapper__.


### registerUnstakeToken

```rust
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(registerUnstakeToken)]
    fn register_unstake_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    );  
```

A setup endpoint, that receives an exact payment of __0.05 EGLD__ (amount required for the deployment of a new token) and that issues and sets all roles for the newly created token. Being a non-fungible ESDT, the __unstake_token__ is handled through a __NonFungibleTokenMapper__.


### setStateActive

```rust
    #[only_owner]
    #[endpoint(setStateActive)]
    fn set_state_active(&self);  
```

A setup endpoint, that updates the state of the contract to __Active__.


### setStateActive

```rust
    #[only_owner]
    #[endpoint(setStateInactive)]
    fn set_state_inactive(&self);  
```

A setup endpoint, that updates the state of the contract to __Inactive__.


## Testing

The contract has been tested through both unit and system tests. Local tests have been done using Rust Testing Framework, which can be found in the _tests_ folder. Here, the tests setup is organized in two folders, _setup_ and _interactions_. The actual testing logic is defined in the _test.rs_ file. In order to replicate the entire workflow of the contract, a __delegation-mock__ contract has been created, that has a basic custom logic that replicates the delegation rewarding system from the protocol level.


## Interaction

The interaction scripts are located in the _interaction_ folder. The scripts are written in python and erdpy is required in order to be used. Interaction scripts are scripts that ease the interaction with the deployed contract by wrapping erdpy sdk functionality in bash scripts. Make sure to update the PEM path and the PROXY and CHAINID values in order to correctly use the scripts.
The testing of this contract has been conducted on the Devnet.
