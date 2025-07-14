multiversx_sc::imports!();

use crate::{liquidity_pool, setup};

#[multiversx_sc::module]
pub trait ClaimBackModule:
    setup::config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + liquidity_pool::LiquidityPoolModule
    + setup::vote::VoteModule
{
    #[payable("*")]
    #[endpoint(claimBack)]
    fn claim_back(&self) {
        let caller = self.blockchain().get_caller();
        require!(
            self.locked_vote_balance(&caller).is_empty(),
            "nothing to claim"
        );
        let locked_balance = self.locked_vote_balance(&caller).take();

        let mut claim_back_early = true;
        let current_epoch = self.blockchain().get_block_epoch();
        for proposal in self.voted_proposals(&caller).iter() {
            if self.proposal_end_period(proposal).get() > current_epoch {
                claim_back_early = false;
            }
        }

        require!(
            claim_back_early || (current_epoch >= locked_balance.claim_back),
            "cannot claim yet"
        );

        self.voted_proposals(&caller).clear();

        self.send().direct_esdt(
            &caller,
            &locked_balance.funds.token_identifier,
            locked_balance.funds.token_nonce,
            &locked_balance.funds.amount,
        );
    }
}
