# Liquid Staking Smart Contract


## Abstract

MultiversX is a proof-of-stake blockchain protocol that seeks to offer extremely fast transaction speeds by using adaptive state sharding. By being a proof-of-stake blockchain, it incentives holders to stake their assets (__EGLD__), in order to increase the network's security, by offering them staking rewards at a protocol level. As more and more products appear on the network, it raises the question for the holders, if they should just use their __EGLD__ in the ecosystem, instead of just staking it. This Liquid Staking SC tries to solve this problem by offering users the ability to stake their __EGLD__ in exchange for another token, that can be further implemented in various products within the MultiversX ecosystem, while still receiving the staking rewards from the protocol.


## Introduction

The Liquid Staking Smart Contract allows users to stake their __EGLD__ in return of __lsEGLD__, a fungible ESDT that can be used in multiple ways in the MultiversX ecosystem, all while retaining the standard staking rewards. It offers users the possibility to stake or unstake their __EGLD__, while rewards are cumulated and redelegated. This means that, in time, the value of __lsEGLD__ will only continue to outgrow that of the __EGLD__, as rewards are compounded every epoch. Also, it offers delegation contract owners means to maintain the status of their delegation contracts, through a whitelisted admin-only endpoint.

## Important note

The Liquid Staking SC is designed to work with both user addresses and other smart contracts. That being said, it is important to note that the contracts that interact with the Liquid Staking SC need to be payable or payable by SC, otherwise any interaction may result in __loss of funds__.


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

The ```unbondTokens``` endpoint allows users to receive back their EGLD after the unbond period has passed. It receives only the payment of __unstake_token_NFT__, where it first checks if the unbond period has passed, and then it reads the delegation contract from where the tokens were unstaked and how much the users must receive. After that, it sends an async call to that contract, hooked with the ```withdraw_tokens_callback```.

In the callback, in case of a succesful result, the reserves storage is updated, the NFT is burnt and the __EGLD__ tokens are sent to the caller. In case of an unsuccesful result, the NFT is sent back to the user.


### claimRewards

```rust
    #[endpoint(claimRewards)]
    fn claim_rewards(&self);
```

The ```claimRewards``` endpoint is callable by any user, and allows each epoch to claim all pending rewards and store them in a __rewards_reserve__ storage, until those rewards are redelegated and are taken into account in the general __virtual_egld_reserve__ storage.
The ```claimRewards``` function implements an ongoing operation mechanism, that claims rewards from delegation contracts until the list is completely covered or until the transaction runs out of gas. In this case, it saves the current iteration of the delegation contract list, being able to later continue from where it left off in a secondary call of the same endpoint. After all delegation contracts have been covered, the claim operation status is set to __Finished__.

The available claim status types are:

```rust
pub enum ClaimStatusType {
    None,
    Pending,
    Finished,
    Delegable,
    Insufficient,
    Redelegated,
}
```

The workflow is as follows:
- In order to start a new ```claimRewards``` operation, the previous claim status must be __Insufficient__ or __Redelegated__. Once the operation has started, the new claim operation is updated to the __Pending__ status. After the claim operation has finished, it is marked with the __Finished__ status. The ```recomputeTokenReserve``` endpoint then updates the rewards storage values, and in case the total available rewards are greater than the minimum delegation amount required (__1 EGLD__), the status is then updated to __Delegable__, otherwise it is updated to __Insufficient__. The ```delegateRewards``` endpoint is then callable (only if the claim status is __Delegable__), which then updates the status to __Redelegated__, allowing the cycle to start once again. In case the status of the claim operation is __Insufficient__ at the end of the rewards reserve recomputation, a new claim operation can be started the next epoch, without any further steps.


### recomputeTokenReserve

```rust
    #[endpoint(recomputeTokenReserve)]
    fn recompute_token_reserve(&self);
```

The ```recomputeTokenReserve``` is an endpoint that allows for the rewards reserve recomputation. It is a mandatory step after the ```claimRewards``` endpoint and it looks at the initial __EGLD__ balance of the contract (the one that was saved at the beginning of the claim operation), compares it with the current balance of the contract (the one after the claim operation has finised claiming the rewards from all the delegation contracts) and saves the difference in the __rewards_reserve__ storage. It also takes into account the __withdrawn_egld__ that users may have unbonded. In case the newly obtained rewards reserve is bigger that the minimum delegation amount, then the claim status is updated to __Delegable__. Otherwise, it is updated to __Insufficient__, in which case a new claim operation can be started the next epoch, without any further steps regarding the rewards. 

The endpoint is callable by any user and can be called only when the claim operation is in the __Finished__ status. 


### delegateRewards

```rust
    #[endpoint(delegateRewards)]
    fn delegate_rewards(&self);
```

As stated before, the ```delegateRewards``` endpoint allows for the delegation of all the cumulated rewards from the ```claimRewards``` endpoint, but only if the rewards amount is greater than the minimum amount required (marked by the __Delegable__ claim status). If all check conditions are met, then a new delegation contract is chosen from the whitelist and a new delegation operation is called through an async call hooked with the ```delegate_rewards_callback```.

In the callback, if the result is succesful, the storage is updated accordingly, adding the __rewards_reserve__ value to the __virtual_egld_reserve__, which in turn increases the value of the __lsEGLD__, compared to the __EGLD__ token.


### whitelistDelegationContract

```rust
    #[only_owner]
    #[endpoint(whitelistDelegationContract)]
    fn whitelist_delegation_contract(
        &self,
        contract_address: ManagedAddress,
        admin_address: ManagedAddress,
        total_staked: BigUint,
        delegation_contract_cap: BigUint,
        nr_nodes: u64,
        apy: u64,
    );
```

Endpoint that allows the owner to whitelist a delegation contract with a set of parameters, sent as arguments (__DelegationContractData__). From the list below, the first 5 variables are user updatable, while __total_staked_from_ls_contract__ and __total_undelegated_from_ls_contract__ variables are automatically updated throughout the contract's workflow.

```rust
pub struct DelegationContractData {
    pub admin_address: ManagedAddress,
    pub total_staked: BigUint,
    pub delegation_contract_cap: BigUint,
    pub nr_nodes: u64,
    pub apy: u64,
    pub total_staked_from_ls_contract: BigUint,
    pub total_undelegated_from_ls_contract: BigUint,
}
```


### changeDelegationContractAdmin
```rust
    #[only_owner]
    #[endpoint(changeDelegationContractAdmin)]
    fn change_delegation_contract_admin(
        &self,
        contract_address: ManagedAddress,
        admin_address: ManagedAddress,
    )
```

Endpoint that allows the owner to update the admin of a specific delegation contract. It takes as arguments the address of the delegation contract and the address of the new admin.


### changeDelegationContractParams

```rust
    #[endpoint(changeDelegationContractParams)]
    fn change_delegation_contract_params(
        &self,
        contract_address: ManagedAddress,
        total_staked: BigUint,
        delegation_contract_cap: BigUint,
        nr_nodes: u64,
        apy: u64,
    );
```

Endpoint that allows the admin of a whitelisted delegation contract to update the given parameters, by sending them as arguments. The caller of the endpoint must be the same as the admin_address that was previously saved for that said delegation contract.


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

The contract has been tested through both unit and system tests. Local tests have been done using Rust Testing Framework, which can be found in the _tests_ folder. Here, the testing setup is organized in two folders, _setup_ and _interactions_. The actual testing logic is defined in the _test.rs_ file. In order to replicate the entire workflow of the contract, a __delegation-mock__ contract has been created, that has a basic custom logic that replicates the delegation rewarding system from the protocol level.


## Interaction

The interaction scripts are located in the _interaction_ folder. The scripts are written in python and erdpy is required in order to be used. Interaction scripts are scripts that ease the interaction with the deployed contract by wrapping erdpy sdk functionality in bash scripts. Make sure to update the PEM path and the PROXY and CHAINID values in order to correctly use the scripts.

The testing of this contract has been conducted on the Devnet.
