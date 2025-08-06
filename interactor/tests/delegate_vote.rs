use interactor::Config;
use interactor::Interact;
use multiversx_sc_snippets::hex;
use multiversx_sc_snippets::imports::*;

const HASH_LENGTH: usize = 32;
const ROOT_HASH: &[u8; 64] = b"9eb26c38568e3bad298efa9adf80c01f3e3539c19a9cd3c77652f33bbe5055ae";

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_delegate_vote() {
    // to fix
    let mut interact = Interact::new(Config::chain_simulator_config()).await;
    let owner_address = Bech32Address::from(interact.owner_address.clone());
    interact.deploy().await;
    interact.deploy_delegation_contract().await;
    interact.deploy_governance_contract().await;
    interact
        .whitelist_delegation_contract(
            1_000_000_000_000_000_000u128,
            interact.state.delegation_address().clone(),
            owner_address.clone(),
            0u128,
            2_000_000_000_000_000_000_000u128,
            1u64,
            50_000u64,
        )
        .await;
    interact.set_state_active().await;
    let _ = interact
        .register_ls_token("LIQTEST", "LTST", 18u32, 50_000_000_000_000_000u128)
        .await;
    let _ = interact
        .register_unstake_token("UNSTAKETEST", "UNTST", 18u32, 50_000_000_000_000_000u128)
        .await;

    let mut times = 0u32;
    loop {
        interact
            .add_liquidity(owner_address.clone(), 12_500_000_000_000_000_000u128)
            .await;
        times += 1;
        if times == 100u32 {
            break;
        }
    }
    interact.deploy_vote_contract().await;

    let root_hash = hex::decode(ROOT_HASH).unwrap();
    let proposal_id = 1u32;
    interact
        .set_root_hash(
            ManagedByteArray::new_from_bytes(root_hash.as_slice().try_into().unwrap()),
            proposal_id,
        )
        .await;

    let proof = get_proof();
    interact
        .delegate_vote(
            owner_address,
            proposal_id,
            "Yes",
            1_250_000_000_000_000_000_000u128,
            proof,
            None,
        )
        .await;
}

fn get_proof() -> Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>> {
    let mut proof = Vec::new();

    // Proof element 1
    let proof1_bytes =
        hex::decode("54103ff9430b9989523224c275f6989b4e9b9eb4f84f540be1a96e37995d5325").unwrap();
    let proof1 = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
        &proof1_bytes.as_slice().try_into().unwrap(),
    );
    proof.push(proof1);

    // Proof element 2
    let proof2_bytes =
        hex::decode("75a60affb12ff780a4c43f98f74866bf56ffd7f08001a0f54ad9a0b867978c39").unwrap();
    let proof2 = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
        &proof2_bytes.as_slice().try_into().unwrap(),
    );
    proof.push(proof2);

    // Proof element 3
    let proof3_bytes =
        hex::decode("14e1bb4cf267d4da3be8eb75b14cc9410f90ed13736a85be8e77a45234aa06c8").unwrap();
    let proof3 = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
        &proof3_bytes.as_slice().try_into().unwrap(),
    );
    proof.push(proof3);

    // Proof element 4
    let proof4_bytes =
        hex::decode("76594f04e196de2839e19bb8139f79cb9045200559598a9d0cc367667373a455").unwrap();
    let proof4 = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
        &proof4_bytes.as_slice().try_into().unwrap(),
    );
    proof.push(proof4);

    // Proof element 5
    let proof5_bytes =
        hex::decode("479f7b3c72ac2bc2b86f5491b95cb0afd5e71df169abb8b08ddb16562f9ca8d3").unwrap();
    let proof5 = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
        &proof5_bytes.as_slice().try_into().unwrap(),
    );
    proof.push(proof5);

    // Proof element 6
    let proof6_bytes =
        hex::decode("9c3046fb46d0f647165e9d71c8d70fe08c528fc5a7c2d5f8d053bb3f79cca974").unwrap();
    let proof6 = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
        &proof6_bytes.as_slice().try_into().unwrap(),
    );
    proof.push(proof6);

    // Proof element 7
    let proof7_bytes =
        hex::decode("405bb9e4317476f12c7cd80fc12936ec22cc49df5cd781fc65173d3605786fd6").unwrap();
    let proof7 = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
        &proof7_bytes.as_slice().try_into().unwrap(),
    );
    proof.push(proof7);

    // Proof element 8
    let proof8_bytes =
        hex::decode("d859cc3acac6c042261c7a66caeeee197284e84739a0a56a5f8eed6636639c3a").unwrap();
    let proof8 = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
        &proof8_bytes.as_slice().try_into().unwrap(),
    );
    proof.push(proof8);

    // Proof element 9
    let proof9_bytes =
        hex::decode("18c4f5656785a4da2e14ab75217dcf57e9da381176cee02e6c9b9b9c78e3c727").unwrap();
    let proof9 = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
        &proof9_bytes.as_slice().try_into().unwrap(),
    );
    proof.push(proof9);

    // Proof element 10
    let proof10_bytes =
        hex::decode("6900c366214a74625ac7a61d304f3a7b819f78f472af8fa2347a191981b0701e").unwrap();
    let proof10 = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
        &proof10_bytes.as_slice().try_into().unwrap(),
    );
    proof.push(proof10);

    // Proof element 11
    let proof11_bytes =
        hex::decode("6a2f1eff54e491cc3eae8085c781291f9870b94c0c1330b54728f41044c2b3f3").unwrap();
    let proof11 = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
        &proof11_bytes.as_slice().try_into().unwrap(),
    );
    proof.push(proof11);

    // Proof element 12
    let proof12_bytes =
        hex::decode("1596066061a71b0674b17c637dd9a23f8869416c59d50675180487ca784881c6").unwrap();
    let proof12 = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
        &proof12_bytes.as_slice().try_into().unwrap(),
    );
    proof.push(proof12);

    proof
}
