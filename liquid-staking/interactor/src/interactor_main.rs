use multiversx_sc_snippets::imports::*;
use rust_interact::liquid_staking_cli;

#[tokio::main]
async fn main() {
    liquid_staking_cli().await;
}
