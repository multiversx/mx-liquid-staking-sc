pub type GasLimit = u64;
pub type Blocks = u64;

pub const DEFAULT_MIN_GAS_TO_SAVE_PROGRESS: GasLimit = 30_000_000;
pub const DEFAULT_GAS_TO_CLAIM_REWARDS: GasLimit = 6_000_000;
pub const MIN_GAS_FOR_ASYNC_CALL: GasLimit = 12_000_000;
pub const MIN_GAS_FOR_CALLBACK: GasLimit = 12_000_000;
pub const MIN_GAS_FINISH_EXEC: GasLimit = 20_000_000;

pub const MIN_BLOCKS_BEFORE_CLEAR_ONGOING_OP: Blocks = 10;
pub const RECOMPUTE_BLOCK_OFFSET: Blocks = 10;

pub const MIN_EGLD_TO_DELEGATE: u64 = 1_000_000_000_000_000_000; // 1 EGLD
pub const EGLD_TO_WHITELIST: u64 = 1_000_000_000_000_000_000; // 1 EGLD

pub const MINIMUM_LIQUIDITY: u64 = 1_000;

pub const MAX_DELEGATION_ADDRESSES: usize = 20;
