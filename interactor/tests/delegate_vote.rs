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
            "yes",
            1_250_000_000_000_000_000_000u128,
            proof,
            None,
        )
        .await;
}

fn get_proof() -> Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>> {
    let mut proof = Vec::new();

    let proof_bytes = vec![
        "54103ff9430b9989523224c275f6989b4e9b9eb4f84f540be1a96e37995d5325",
        "75a60affb12ff780a4c43f98f74866bf56ffd7f08001a0f54ad9a0b867978c39",
        "14e1bb4cf267d4da3be8eb75b14cc9410f90ed13736a85be8e77a45234aa06c8",
        "76594f04e196de2839e19bb8139f79cb9045200559598a9d0cc367667373a455",
        "479f7b3c72ac2bc2b86f5491b95cb0afd5e71df169abb8b08ddb16562f9ca8d3",
        "9c3046fb46d0f647165e9d71c8d70fe08c528fc5a7c2d5f8d053bb3f79cca974",
        "405bb9e4317476f12c7cd80fc12936ec22cc49df5cd781fc65173d3605786fd6",
        "d859cc3acac6c042261c7a66caeeee197284e84739a0a56a5f8eed6636639c3a",
        "18c4f5656785a4da2e14ab75217dcf57e9da381176cee02e6c9b9b9c78e3c727",
        "6900c366214a74625ac7a61d304f3a7b819f78f472af8fa2347a191981b0701e",
        "6a2f1eff54e491cc3eae8085c781291f9870b94c0c1330b54728f41044c2b3f3",
        "1596066061a71b0674b17c637dd9a23f8869416c59d50675180487ca784881c6",
    ];

    for bytes in proof_bytes {
        let hex_bytes = hex::decode(bytes).unwrap();
        let managed_array_bytes = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
            &hex_bytes.as_slice().try_into().unwrap(),
        );
        proof.push(managed_array_bytes);
    }

    proof
}
