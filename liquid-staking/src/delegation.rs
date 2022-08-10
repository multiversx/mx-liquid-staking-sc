use crate::errors::{ERROR_ALREADY_WHITELISTED, ERROR_NOT_WHITELISTED};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Debug)]
pub struct DelegationContractData<M: ManagedTypeApi> {
    pub total_staked: BigUint<M>,
    pub delegation_contract_cap: u64,
    pub nr_nodes: u64,
    pub apy: u64,
    pub total_staked_from_ls_contract: BigUint<M>,
    pub total_undelegated_from_ls_contract: BigUint<M>,
}

#[elrond_wasm::module]
pub trait DelegationModule: crate::config::ConfigModule {
    #[only_owner]
    #[endpoint(whitelistDelegationContract)]
    fn whitelist_delegation_contract(
        &self,
        contract_address: ManagedAddress,
        total_staked: BigUint,
        delegation_contract_cap: u64,
        nr_nodes: u64,
        apy: u64,
    ) {
        require!(
            self.delegation_sc_address(&contract_address).is_empty(),
            ERROR_ALREADY_WHITELISTED
        );

        let contract_data = DelegationContractData {
            total_staked,
            delegation_contract_cap,
            nr_nodes,
            apy,
            total_staked_from_ls_contract: BigUint::zero(),
            total_undelegated_from_ls_contract: BigUint::zero(),
        };

        self.delegation_sc_address(&contract_address)
            .set(contract_data);
        self.delegation_sc_addresses_list()
            .push_back(contract_address);
    }

    #[only_owner]
    #[endpoint(changeDelegationContractParams)]
    fn change_delegation_contract_params(
        &self,
        contract_address: ManagedAddress,
        total_staked: BigUint,
        delegation_contract_cap: u64,
        nr_nodes: u64,
        apy: u64,
    ) {
        require!(
            !self.delegation_sc_address(&contract_address).is_empty(),
            ERROR_NOT_WHITELISTED
        );

        let delegation_sc_address_mapper = self.delegation_sc_address(&contract_address);
        let old_contract_data = delegation_sc_address_mapper.get();
        let new_contract_data = DelegationContractData {
            total_staked,
            delegation_contract_cap,
            nr_nodes,
            apy,
            total_staked_from_ls_contract: old_contract_data.total_staked_from_ls_contract,
            total_undelegated_from_ls_contract: old_contract_data
                .total_undelegated_from_ls_contract,
        };

        delegation_sc_address_mapper.set(new_contract_data);
    }

    // TODO - add check for available delegation space
    // Round Robin
    fn get_next_delegation_contract(&self) -> ManagedAddress<Self::Api> {
        let new_address;
        let first_address = self.delegation_sc_addresses_list().pop_front();
        match first_address {
            Some(first_address) => {
                new_address = first_address.into_value();
                self.delegation_sc_addresses_list()
                    .push_back(new_address.clone());
                new_address
            }
            None => ManagedAddress::zero(),
        }
    }

    #[view(getDelegationScAddressesList)]
    #[storage_mapper("delegation_sc_addresses_list")]
    fn delegation_sc_addresses_list(&self) -> LinkedListMapper<ManagedAddress>;

    #[view(getDelegationScAddress)]
    #[storage_mapper("delegation_sc_address")]
    fn delegation_sc_address(
        &self,
        contract_address: &ManagedAddress,
    ) -> SingleValueMapper<DelegationContractData<Self::Api>>;
}
