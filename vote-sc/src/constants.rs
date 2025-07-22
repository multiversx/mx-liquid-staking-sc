multiversx_sc::imports!();

pub const HASH_LENGTH: usize = 32;
pub const PROOF_LENGTH: usize = 18;
pub type ProposalId = u32;
pub type Hash<M> = ManagedByteArray<M, HASH_LENGTH>;
