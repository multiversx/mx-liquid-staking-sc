elrond_wasm::imports!();

#[elrond_wasm::proxy]
pub trait DelegationProxy {
    #[payable("EGLD")]
    #[endpoint(delegate)]
    fn delegate(&self);

    #[endpoint(unDelegate)]
    fn unDelegate(&self, egld_amount: BigUint);

    #[endpoint(withdraw)]
    fn withdraw(&self);
}