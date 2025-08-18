use delegation_sc_interact::DelegateCallsInteract;
use multiversx_sc_snippets::imports::*;

use crate::Interact;

impl Interact {
    pub async fn deploy_delegation_contract(&mut self) {
        let mut delegation_interactor =
            DelegateCallsInteract::new(delegation_sc_interact::Config::chain_simulator_config())
                .await;
        let validator_1 = Validator::from_pem_file("./../delegation/interactor/validatorKey1.pem")
            .expect("unable to load validator key");
        let validator_2 = Validator::from_pem_file("./../delegation/interactor/validatorKey2.pem")
            .expect("unable to load validator key");

        let _ = delegation_interactor
            .interactor
            .add_key(validator_1.private_key.clone())
            .await
            .unwrap();
        let _ = delegation_interactor
            .interactor
            .add_key(validator_2.private_key.clone())
            .await
            .unwrap();
        delegation_interactor
            .set_state(&delegation_interactor.owner.to_address())
            .await;
        delegation_interactor
            .set_state(&delegation_interactor.delegator1.to_address())
            .await;
        delegation_interactor
            .set_state(&delegation_interactor.delegator2.to_address())
            .await;
        delegation_interactor
            .create_new_delegation_contract(0, 3745u64, 1_250_000_000_000_000_000_000u128)
            .await;
        delegation_interactor
            .set_check_cap_on_redelegate_rewards(false)
            .await;

        let addresses = delegation_interactor.get_all_contract_addresses().await;
        assert_eq!(
            &addresses[0],
            delegation_interactor.state.current_delegation_address()
        );

        delegation_interactor
            .add_nodes(vec![
                (validator_1.public_key, BLSSignature::dummy("signed1")),
                (validator_2.public_key, BLSSignature::dummy("signed2")),
            ])
            .await;

        let new_address_bech32 = &addresses[0];
        self.state
            .set_delegation_address(new_address_bech32.clone());

        let new_address_string = new_address_bech32.to_string();
        println!("new delegation address: {new_address_string}");
    }
}
