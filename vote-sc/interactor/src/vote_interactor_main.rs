use multiversx_sc_snippets::imports::*;
use vote_interact::vote_sc_cli;

#[tokio::main]
async fn main() {
    vote_sc_cli().await;
}
