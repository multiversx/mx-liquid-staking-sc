multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Eq, Copy, Clone, Debug)]
pub enum VoteType {
    Yes = 0,
    No = 1,
    Veto = 2,
    Abstain = 3,
}
