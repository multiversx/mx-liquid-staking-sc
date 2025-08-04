multiversx_sc::imports!();

pub const HASH_LENGTH: usize = 32;
pub const PROOF_LENGTH: usize = 18;
pub const SYNC_CALL_GAS_LIMIT: u64 = 10_000;
pub type ProposalId = u32;
pub type Hash<M> = ManagedByteArray<M, HASH_LENGTH>;
