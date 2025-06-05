pub static ERROR_NOT_ACTIVE: &[u8] = b"Not active";
pub static ERROR_LS_TOKEN_NOT_ISSUED: &[u8] = b"LS token not issued";
pub static ERROR_DELEGATION_CONTRACT_NOT_INITIALIZED: &[u8] =
    b"Delegation contract was not initialized yet";
pub static ERROR_INSUFFICIENT_GAS: &[u8] = b"Insufficient gas remaining for the callback";

pub static ERROR_CLAIM_IN_PROGRESS: &[u8] = b"Claim operation is already in progress";
pub static ERROR_OLD_CLAIM_START: &[u8] =
    b"Previous claimed rewards must be redelegated or lesser than 1 EGLD";
pub static ERROR_CLAIM_REDELEGATE: &[u8] = b"Old claimed rewards must be greater than 1 EGLD";
pub static ERROR_RECOMPUTE_RESERVES: &[u8] = b"Claim operation must be in the finished status";
pub static ERROR_RECOMPUTE_TOO_SOON: &[u8] = b"Recompute operation called too soon";
pub static ERROR_CLAIM_EPOCH: &[u8] = b"The rewards were already claimed for this epoch";
pub static ERROR_UNSTAKE_PERIOD_NOT_PASSED: &[u8] = b"The unstake period has not passed";

pub static ERROR_BAD_PAYMENT_TOKEN: &[u8] = b"Bad payment token";
pub static ERROR_BAD_PAYMENT_AMOUNT: &[u8] = b"Insufficient delegated amount";
pub static ERROR_BAD_WHITELIST_FEE: &[u8] = b"Whitelisting contracts costs 1 EGLD";
pub static ERROR_INSUFFICIENT_UNSTAKE_AMOUNT: &[u8] = b"Insufficient unstake amount";
pub static ERROR_INSUFFICIENT_LIQUIDITY: &[u8] = b"Insufficient liquidity minted";
pub static ERROR_INSUFFICIENT_LIQ_BURNED: &[u8] = b"Insufficient liquidity burned";

pub static ERROR_BAD_DELEGATION_ADDRESS: &[u8] = b"No delegation contract available";
pub static ERROR_BAD_DELEGATION_AMOUNT: &[u8] = b"Delegation amount must be at least 1 EGLD";
pub static ERROR_NO_DELEGATION_CONTRACTS: &[u8] = b"There are no delegation contracts whitelisted";
pub static ERROR_FIRST_DELEGATION_NODE: &[u8] = b"The first delegation node is incorrect";
pub static ERROR_ALREADY_WHITELISTED: &[u8] = b"Delegation contract already whitelisted";
pub static ERROR_NOT_WHITELISTED: &[u8] = b"Delegation contract is not whitelisted";
pub static ERROR_VOTE_SC_NOT_SET: &[u8] = b"Vote contract is not set";
pub static ERROR_DELEGATION_CAP: &[u8] =
    b"Delegation cap must be higher than the total staked amount";
pub static ERROR_ONLY_DELEGATION_ADMIN: &[u8] =
    b"Only the admin of the delegation contract can change the status";
