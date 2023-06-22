multiversx_sc::imports!();

#[multiversx_sc::proxy]
pub trait DelegationProxy {
    #[payable("EGLD")]
    #[endpoint(delegate)]
    fn delegate(&self);

    #[endpoint(unDelegate)]
    fn undelegate(&self, egld_amount: BigUint);

    #[endpoint(withdraw)]
    fn withdraw(&self);

    #[endpoint(claimRewards)]
    fn claim_rewards(&self);
}
