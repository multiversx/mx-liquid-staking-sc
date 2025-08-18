multiversx_sc::imports!();
pub type GasLimit = u64;

pub const HASH_LENGTH: usize = 32;
pub const MIN_GAS_FOR_SYNC_CALL: GasLimit = 15_000_000;
pub const MIN_GAS_FINISH_EXEC: GasLimit = 1_000_000;
pub type ProposalId = u32;
pub type Hash<M> = ManagedByteArray<M, HASH_LENGTH>;
