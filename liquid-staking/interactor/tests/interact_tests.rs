use multiversx_sc_snippets::imports::*;
use rust_interact::ContractInteract;

// Simple deploy test that runs on the real blockchain configuration.
// In order for this test to work, make sure that the `config.toml` file contains the real blockchain config (or choose it manually)
// Can be run with `sc-meta test`.
#[tokio::test]
#[cfg_attr(not(feature = "blockchain-tests"), ignore)]
async fn deploy_test_liquid_staking() {
    let mut interactor = ContractInteract::new().await;

    interactor.deploy().await;
}
