use crate::errors::{
    ERROR_ALREADY_WHITELISTED, ERROR_BAD_DELEGATION_ADDRESS, ERROR_CLAIM_EPOCH,
    ERROR_CLAIM_IN_PROGRESS, ERROR_CLAIM_START, ERROR_DELEGATION_CAP, ERROR_FIRST_DELEGATION_NODE,
    ERROR_NOT_WHITELISTED, ERROR_NO_DELEGATION_CONTRACTS, ERROR_OLD_CLAIM_START,
    ERROR_ONLY_DELEGATION_ADMIN,
};
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const MAX_DELEGATION_ADDRESSES: usize = 50;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, TypeAbi, Clone)]
pub enum ClaimStatusType {
    None,
    Pending,
    Finished,
    Delegable,
    Insufficient,
    Redelegated,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, TypeAbi, Clone)]
pub struct ClaimStatus<M: ManagedTypeApi> {
    pub status: ClaimStatusType,
    pub last_claim_epoch: u64,
    pub last_claim_block: u64,
    pub current_node: u32,
    pub starting_token_reserve: BigUint<M>,
}

impl<M: ManagedTypeApi> Default for ClaimStatus<M> {
    fn default() -> Self {
        Self {
            status: ClaimStatusType::None,
            last_claim_epoch: 0,
            last_claim_block: 0,
            current_node: 0,
            starting_token_reserve: BigUint::zero(),
        }
    }
}

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub struct DelegationContractData<M: ManagedTypeApi> {
    pub admin_address: ManagedAddress<M>,
    pub total_staked: BigUint<M>,
    pub delegation_contract_cap: BigUint<M>,
    pub nr_nodes: u64,
    pub apy: u64,
    pub total_staked_from_ls_contract: BigUint<M>,
    pub total_unstaked_from_ls_contract: BigUint<M>,
    pub total_unbonded_from_ls_contract: BigUint<M>,
}

#[multiversx_sc::module]
pub trait DelegationModule:
    crate::config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + multiversx_sc_modules::ongoing_operation::OngoingOperationModule
{
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
    ) {
        require!(
            self.delegation_contract_data(&contract_address).is_empty(),
            ERROR_ALREADY_WHITELISTED
        );

        require!(
            delegation_contract_cap >= total_staked,
            ERROR_DELEGATION_CAP
        );

        let contract_data = DelegationContractData {
            admin_address,
            total_staked,
            delegation_contract_cap,
            nr_nodes,
            apy,
            total_staked_from_ls_contract: BigUint::zero(),
            total_unstaked_from_ls_contract: BigUint::zero(),
            total_unbonded_from_ls_contract: BigUint::zero(),
        };

        self.delegation_contract_data(&contract_address)
            .set(contract_data);
        self.add_and_order_delegation_address_in_list(contract_address, apy);
        require!(
            self.delegation_addresses_list().len() <= MAX_DELEGATION_ADDRESSES,
            "Maximum number of delegation addresses reached"
        );
    }

    #[only_owner]
    #[endpoint(changeDelegationContractAdmin)]
    fn change_delegation_contract_admin(
        &self,
        contract_address: ManagedAddress,
        admin_address: ManagedAddress,
    ) {
        let delegation_address_mapper = self.delegation_contract_data(&contract_address);
        require!(!delegation_address_mapper.is_empty(), ERROR_NOT_WHITELISTED);

        delegation_address_mapper.update(|contract_data| {
            contract_data.admin_address = admin_address;
        });
    }

    #[endpoint(changeDelegationContractParams)]
    fn change_delegation_contract_params(
        &self,
        contract_address: ManagedAddress,
        total_staked: BigUint,
        delegation_contract_cap: BigUint,
        nr_nodes: u64,
        apy: u64,
    ) {
        let caller = self.blockchain().get_caller();
        let delegation_address_mapper = self.delegation_contract_data(&contract_address);
        let old_contract_data = delegation_address_mapper.get();
        require!(!delegation_address_mapper.is_empty(), ERROR_NOT_WHITELISTED);
        require!(
            old_contract_data.admin_address == caller,
            ERROR_ONLY_DELEGATION_ADMIN
        );
        require!(
            delegation_contract_cap >= total_staked,
            ERROR_DELEGATION_CAP
        );

        if old_contract_data.apy != apy {
            self.remove_delegation_address_from_list(&contract_address);
            self.add_and_order_delegation_address_in_list(contract_address, apy)
        }

        delegation_address_mapper.update(|contract_data| {
            contract_data.total_staked = total_staked;
            contract_data.delegation_contract_cap = delegation_contract_cap;
            contract_data.nr_nodes = nr_nodes;
            contract_data.apy = apy;
        });
    }

    fn add_address_to_be_claimed(&self, contract_address: ManagedAddress) {
        if self.addresses_to_claim().is_empty() {
            self.addresses_to_claim().push_front(contract_address);
        } else {
            self.addresses_to_claim().push_back(contract_address);
        }
    }

    fn add_and_order_delegation_address_in_list(&self, contract_address: ManagedAddress, apy: u64) {
        let mut delegation_addresses_mapper = self.delegation_addresses_list();
        if delegation_addresses_mapper.is_empty() {
            delegation_addresses_mapper.push_front(contract_address);
        } else {
            let mut check_if_added = false;
            for delegation_address_element in delegation_addresses_mapper.iter() {
                let node_id = delegation_address_element.get_node_id();
                let delegation_address = delegation_address_element.into_value();
                let delegation_contract_data =
                    self.delegation_contract_data(&delegation_address).get();
                if apy >= delegation_contract_data.apy {
                    self.delegation_addresses_list()
                        .push_before_node_id(node_id, contract_address.clone());
                    check_if_added = true;
                    break;
                }
            }
            if !check_if_added {
                delegation_addresses_mapper.push_back(contract_address);
            }
        }
    }

    fn remove_delegation_address_from_list(&self, contract_address: &ManagedAddress) {
        for delegation_address_element in self.delegation_addresses_list().iter() {
            let node_id = delegation_address_element.get_node_id();
            let delegation_address = delegation_address_element.into_value();
            if contract_address == &delegation_address {
                self.delegation_addresses_list().remove_node_by_id(node_id);
                break;
            }
        }
    }

    fn move_delegation_contract_to_back(&self, delegation_contract: ManagedAddress) {
        self.remove_delegation_address_from_list(&delegation_contract);
        self.delegation_addresses_list()
            .push_back(delegation_contract);
    }

    fn get_delegation_contract_for_delegate(
        &self,
        amount_to_delegate: &BigUint,
    ) -> ManagedAddress<Self::Api> {
        require!(
            !self.delegation_addresses_list().is_empty(),
            ERROR_NO_DELEGATION_CONTRACTS
        );

        let delegation_addresses_mapper = self.delegation_addresses_list();

        for delegation_address_element in delegation_addresses_mapper.iter() {
            let delegation_address = delegation_address_element.into_value();
            let delegation_contract_data = self.delegation_contract_data(&delegation_address).get();

            let delegation_space_left = &delegation_contract_data.delegation_contract_cap
                - &delegation_contract_data.total_staked;
            if amount_to_delegate <= &delegation_space_left {
                return delegation_address;
            }
        }
        sc_panic!(ERROR_BAD_DELEGATION_ADDRESS);
    }

    fn get_delegation_contract_for_undelegate(
        &self,
        amount_to_undelegate: &BigUint,
    ) -> ManagedAddress<Self::Api> {
        require!(
            !self.delegation_addresses_list().is_empty(),
            ERROR_NO_DELEGATION_CONTRACTS
        );

        let delegation_addresses_mapper = self.delegation_addresses_list();

        for delegation_address_element in delegation_addresses_mapper.iter() {
            let delegation_address = delegation_address_element.into_value();
            let delegation_contract_data = self.delegation_contract_data(&delegation_address).get();

            if &delegation_contract_data.total_staked_from_ls_contract >= amount_to_undelegate {
                return delegation_address;
            }
        }
        sc_panic!(ERROR_BAD_DELEGATION_ADDRESS);
    }

    fn check_claim_operation(
        &self,
        current_claim_status: &ClaimStatus<Self::Api>,
        old_claim_status: ClaimStatus<Self::Api>,
        current_epoch: u64,
    ) {
        require!(
            current_claim_status.status == ClaimStatusType::None
                || current_claim_status.status == ClaimStatusType::Pending,
            ERROR_CLAIM_START
        );
        require!(
            old_claim_status.status == ClaimStatusType::Redelegated
                || old_claim_status.status == ClaimStatusType::Insufficient,
            ERROR_OLD_CLAIM_START
        );
        require!(
            current_epoch > old_claim_status.last_claim_epoch,
            ERROR_CLAIM_EPOCH
        );
    }

    fn prepare_claim_operation(
        &self,
        current_claim_status: &mut ClaimStatus<Self::Api>,
        current_epoch: u64,
    ) {
        let mut delegation_addresses_mapper = self.delegation_addresses_list();
        if current_claim_status.status == ClaimStatusType::None {
            require!(
                delegation_addresses_mapper.front().unwrap().get_node_id() != 0,
                ERROR_FIRST_DELEGATION_NODE
            );
            require!(
                self.addresses_to_claim().is_empty(),
                ERROR_CLAIM_IN_PROGRESS
            );
            current_claim_status.status = ClaimStatusType::Pending;
            current_claim_status.last_claim_epoch = current_epoch;
            current_claim_status.current_node =
                delegation_addresses_mapper.front().unwrap().get_node_id();
            let current_total_withdrawn_egld = self.total_withdrawn_egld().get();
            current_claim_status.starting_token_reserve = self
                .blockchain()
                .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0)
                - current_total_withdrawn_egld;
        }

        let mut last_node = current_claim_status.current_node;

        while last_node != 0 {
            let current_node = delegation_addresses_mapper
                .get_node_by_id(last_node)
                .unwrap();
            let current_address = current_node.clone().into_value();
            self.add_address_to_be_claimed(current_address);
            last_node = current_node.get_next_node_id();
            delegation_addresses_mapper.remove_node(&current_node);
        }
    }

    #[view(getDelegationStatus)]
    fn get_delegation_status(&self) -> ClaimStatusType {
        let claim_status = self.delegation_claim_status().get();
        claim_status.status
    }

    #[view(getDelegationContractStakedAmount)]
    fn get_delegation_contract_staked_amount(&self, delegation_address: ManagedAddress) -> BigUint {
        let delegation_contract_data = self.delegation_contract_data(&delegation_address).get();
        delegation_contract_data.total_staked_from_ls_contract
    }

    #[view(getDelegationContractUnstakedAmount)]
    fn get_delegation_contract_unstaked_amount(
        &self,
        delegation_address: ManagedAddress,
    ) -> BigUint {
        let delegation_contract_data = self.delegation_contract_data(&delegation_address).get();
        delegation_contract_data.total_unstaked_from_ls_contract
    }

    #[view(getDelegationContractUnbondedAmount)]
    fn get_delegation_contract_unbonded_amount(
        &self,
        delegation_address: ManagedAddress,
    ) -> BigUint {
        let delegation_contract_data = self.delegation_contract_data(&delegation_address).get();
        delegation_contract_data.total_unbonded_from_ls_contract
    }

    #[view(getDelegationAddressesList)]
    #[storage_mapper("delegationAddressesList")]
    fn delegation_addresses_list(&self) -> LinkedListMapper<ManagedAddress>;

    #[view(getAddressesToClaim)]
    #[storage_mapper("addressesToClaim")]
    fn addresses_to_claim(&self) -> LinkedListMapper<ManagedAddress>;

    #[view(getDelegationClaimStatus)]
    #[storage_mapper("delegationClaimStatus")]
    fn delegation_claim_status(&self) -> SingleValueMapper<ClaimStatus<Self::Api>>;

    #[view(getDelegationContractData)]
    #[storage_mapper("delegationContractData")]
    fn delegation_contract_data(
        &self,
        contract_address: &ManagedAddress,
    ) -> SingleValueMapper<DelegationContractData<Self::Api>>;
}
