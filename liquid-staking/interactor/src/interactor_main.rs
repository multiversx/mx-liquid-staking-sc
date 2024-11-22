use multiversx_sc_snippets::imports::*;
use liquid_staking_interactor::liquid_staking_cli;

#[tokio::main]
async fn main() {
    liquid_staking_cli().await;
}  
