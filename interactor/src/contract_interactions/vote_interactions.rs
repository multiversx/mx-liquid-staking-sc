use multiversx_sc_snippets::imports::*;

use crate::{contract_proxies::*, Interact};

const HASH_LENGTH: usize = 32;
const PROOF_LENGTH: usize = 18;

impl Interact {
    pub async fn deploy_vote_contract(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(vote_proxy::VoteSCProxy)
            .init()
            .code(&self.vote_contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;

        let new_address_bech32 = Bech32Address::from(&new_address);
        self.state.set_vote_address(new_address_bech32.clone());

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.vote_address())
            .gas(100_000_000u64)
            .typed(vote_proxy::VoteSCProxy)
            .set_liquid_staking_address(self.state.liquid_staking_address())
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.liquid_staking_address())
            .gas(100_000_000u64)
            .typed(liquid_staking_proxy::LiquidStakingProxy)
            .set_vote_contract(new_address_bech32.clone())
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        let gas = 30_000_000u64;
        let hash = ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
            b"ed013f30ed9e82a734b99aaa014f7193",
        );
        let _ = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.vote_address())
            .gas(gas)
            .typed(vote_proxy::VoteSCProxy)
            .set_root_hash(hash, 1u32)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        let result_value = self
            .interactor
            .query()
            .to(self.state.vote_address())
            .typed(vote_proxy::VoteSCProxy)
            .get_root_hash(1u32)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        let new_address_string = new_address_bech32.to_string();
        println!("new address: {new_address_string}");

        println!("get_root_hash call result: {result_value:?}");
    }

    pub async fn delegate_vote(
        &mut self,
        voter: Bech32Address,
        proposal_id: u32,
        vote: &str,
        voting_power: u128,
        proof: ArrayVec<ManagedByteArray<StaticApi, { HASH_LENGTH }>, { PROOF_LENGTH }>,
        error: Option<ExpectError<'_>>,
    ) {
        let tx = self
            .interactor
            .tx()
            .from(voter)
            .to(self.state.vote_address())
            .gas(50_000_000u64)
            .typed(vote_proxy::VoteSCProxy)
            .delegate_vote(proposal_id, vote, voting_power, proof);

        match error {
            None => {
                tx.returns(ReturnsResultUnmanaged).run().await;
            }
            Some(expect_error) => {
                tx.returns(expect_error).run().await;
            }
        }
    }
}
