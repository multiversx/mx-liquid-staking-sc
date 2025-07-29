use multiversx_sc_snippets::imports::*;
use rust_interact::{vote_interact_config::Config, VoteInteract};

// Simple deploy test that runs using the chain simulator configuration.
// In order for this test to work, make sure that the `config.toml` file contains the chain simulator config (or choose it manually)
// The chain simulator should already be installed and running before attempting to run this test.
// The chain-simulator-tests feature should be present in Cargo.toml.
// Can be run with `sc-meta test -c`.
#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn deploy_test_vote_sc_cs() {
    let mut interactor = VoteInteract::new(Config::chain_simulator_config()).await;

    interactor.deploy().await;
}
