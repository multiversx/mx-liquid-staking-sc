
use multiversx_sc_snippets::imports::*;
use rust_interact::vote_sc_cli;

#[tokio::main]
async fn main() {
    vote_sc_cli().await;
}  

