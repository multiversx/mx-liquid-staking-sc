use interactor::Config;
use interactor::Interact;
use multiversx_sc_snippets::hex;
use multiversx_sc_snippets::imports::*;

const HASH_LENGTH: usize = 32;
const ROOT_HASH: &[u8; 64] = b"078bc8a05f5e62733ca27a4e0df5f5ff2d7327c9ab6c7f4766b6af12b5cc9183";

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

    // delegate_vote attempt with wrong proof

    let wrong_proof = get_wrong_proof();
    interact
        .delegate_vote(
            owner_address.clone(),
            proposal_id,
            "yes",
            1_000_000_000_000_000_000u128,
            wrong_proof,
            Some(ExpectError(4, "Invalid merkle proof provided")),
        )
        .await;

    // delegate_vote attempt with wrong proposal_id

    let wrong_proposal_id = 42u32;
    let proof = get_proof();
    interact
        .delegate_vote(
            owner_address.clone(),
            wrong_proposal_id,
            "yes",
            1_000_000_000_000_000_000u128,
            proof.clone(),
            Some(ExpectError(4, "Invalid root hash provided")),
        )
        .await;

    // delegate_vote attempt with wrong voting power

    interact
        .delegate_vote(
            owner_address.clone(),
            proposal_id,
            "yes",
            2_000_000_000_000_000_000u128,
            proof.clone(),
            Some(ExpectError(4, "Invalid merkle proof provided")),
        )
        .await;

    // delegate_vote that should pass

    interact
        .delegate_vote(
            owner_address,
            proposal_id,
            "yes",
            1_000_000_000_000_000_000u128,
            proof,
            None,
        )
        .await;
}

fn get_proof() -> Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>> {
    let mut proof = Vec::new();

    let proof_bytes = vec![
        "330f8db028b7b5a9435a0ddfd012bd29996fa9e38bfbf65ea32872c3468a06cb",
        "972e54453b055faafc5d24d7486e7377cfce3d82a94f2d1dd6143ae7f9ddd06d",
        "9b3c15e802052c3b7687dc35da074dffc5675501c8f924478cb98c97b92a0db2",
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

fn get_wrong_proof() -> Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>> {
    let mut proof = Vec::new();

    let proof_bytes = vec![
        "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
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
