pub static ERROR_ACTIVE: &[u8] = b"Active state";
pub static ERROR_NOT_ACTIVE: &[u8] = b"Not active";
pub static ERROR_LS_TOKEN_NOT_ISSUED: &[u8] = b"LS token not issued";

pub static ERROR_BAD_DELEGATION_ADDRESS: &[u8] = b"Delegation address wrong value";
pub static ERROR_UNSTAKE_PERIOD_NOT_PASSED: &[u8] = b"The unstake period has not passed";

pub static ERROR_CLAIM_START: &[u8] = b"Claim operation must be new or pending";
pub static ERROR_CLAIM_REDELEGATE: &[u8] = b"Old claimed rewards must be delegated first";
pub static ERROR_CLAIM_EPOCH: &[u8] = b"The rewards were already claimed for this epoch";

pub static ERROR_BAD_PAYMENT_TOKENS: &[u8] = b"Bad payment tokens";
pub static ERROR_BAD_PAYMENT_AMOUNT: &[u8] = b"Bad payment amount";

pub static ERROR_INSUFFICIENT_LIQUIDITY: &[u8] = b"Insufficient liquidity minted";
pub static ERROR_INSUFFICIENT_LIQ_BURNED: &[u8] = b"Insufficient liquidity burned";

pub static ERROR_NOT_ENOUGH_RESERVE: &[u8] = b"Not enough reserve";
pub static ERROR_NOT_ENOUGH_LP: &[u8] = b"Not enough LP token supply";

pub static ERROR_INITIAL_LIQUIDITY_NOT_ADDED: &[u8] = b"Initial liquidity was not added";
pub static ERROR_INITIAL_LIQUIDITY_ALREADY_ADDED: &[u8] = b"Initial liquidity was already added";

pub static ERROR_NO_DELEGATION_CONTRACTS: &[u8] = b"There are no delegation contracts whitelisted";
pub static ERROR_ALREADY_WHITELISTED: &[u8] = b"Delegation contract already whitelisted";
pub static ERROR_NOT_WHITELISTED: &[u8] = b"Delegation contract is not whitelisted";
