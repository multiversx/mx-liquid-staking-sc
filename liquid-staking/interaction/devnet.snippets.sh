WALLET_PEM="/Users/user/DevKitt/walletKey.pem"
WALLET_PEM2="/Users/user/DevKitt/walletKey2.pem"
WALLET_PEM3="/Users/user/DevKitt/walletKey3.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"

LIQUID_STAKING_WASM_PATH="/Users/user/Elrond/sc-liquid-staking-rs/liquid-staking/output/liquid-staking.wasm"

OWNER_ADDRESS="erd14nw9pukqyqu75gj0shm8upsegjft8l0awjefp877phfx74775dsq49swp3"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqdpqkhsz7lmlswgs40zca0dw70tve38mr5dsqxrptfn"

deploySC() {
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${LIQUID_STAKING_WASM_PATH} \
        --pem=${WALLET_PEM} \
        --gas-limit=100000000 \
        --metadata-payable \
        --metadata-payable-by-sc \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

upgradeSC() {
    erdpy --verbose contract upgrade ${CONTRACT_ADDRESS} --recall-nonce \
        --bytecode=${LIQUID_STAKING_WASM_PATH} \
        --pem=${WALLET_PEM} \
        --gas-limit=100000000 \
        --metadata-payable \
        --metadata-payable-by-sc \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

TOKEN_NAME=0x4c53544f4b454e #LSTOKEN
TOKEN_TICKER=0x4c5354 #LST
TOKEN_DECIMALS=18
registerLsToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=100000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --value=50000000000000000 \
        --function="registerLsToken" \
        --arguments ${TOKEN_NAME} ${TOKEN_TICKER} ${TOKEN_DECIMALS} \
        --send || return
}

UNSTAKE_TOKEN_NAME=0x4c53554e5354414b45 #LSUNSTAKE
UNSTAKE_TOKEN_TICKER=0x4c5355 #LSU
registerUnstakeToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=100000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --value=50000000000000000 \
        --function="registerUnstakeToken" \
        --arguments ${UNSTAKE_TOKEN_NAME} ${UNSTAKE_TOKEN_TICKER} ${TOKEN_DECIMALS} \
        --send || return
}

setStateActive() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="setStateActive" \
        --send || return
}

setStateInactive() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="setStateInactive" \
        --send || return
}

###PARAMS
DELEGATION_ADDRESS="erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqq98lllls54qqg7"
TOTAL_STAKED=1200000000000000000000
DELEGATION_CAP=1500000000000000000000 
NR_NODES=3
APY=11000
whitelistDelegationContract() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="whitelistDelegationContract" \
        --arguments ${DELEGATION_ADDRESS} ${OWNER_ADDRESS} ${TOTAL_STAKED} ${DELEGATION_CAP} ${NR_NODES} ${APY}\
        --send || return
}

NEW_TOTAL_STAKED=1500000000000000000000
changeDelegationContractParams() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="changeDelegationContractParams" \
        --arguments ${DELEGATION_ADDRESS} ${NEW_TOTAL_STAKED} ${DELEGATION_CAP} ${NR_NODES} ${APY}\
        --send || return
}

###PARAMS
#1 - Amount
addLiquidity() {
        erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM3} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --value=$1 \
        --function="addLiquidity" \
        --send || return
}

###PARAMS
#1 - Amount
LS_TOKEN=str:LST-208abd
REMOVE_LIQUIDITY_METHOD=str:removeLiquidity
removeLiquidity() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="ESDTTransfer" \
        --arguments ${LS_TOKEN} $1 ${REMOVE_LIQUIDITY_METHOD} \
        --send || return
}

###PARAMS
#1 - Nonce
#2 - Amount
unbondTokens() {
    user_address="$(erdpy wallet pem-address $WALLET_PEM)"
    method_name=str:unbondTokens
    unbond_token=str:LSU-ed6365
    unbond_token_nonce=$1
    unbond_token_amount=$2
    erdpy --verbose contract call $user_address --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="ESDTNFTTransfer" \
        --arguments $unbond_token $1 $2 ${CONTRACT_ADDRESS} $method_name \
        --send || return
}

claimRewards() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="claimRewards" \
        --send || return
}

recomputeTokenReserve() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="recomputeTokenReserve" \
        --send || return
}

delegateRewards() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="delegateRewards" \
        --send || return
}

# VIEWS

getLsTokenId() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getLsTokenId" \
}

getUnstakeTokenId() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getUnstakeTokenId" \
}

getUnstakeTokenSupply() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getUnstakeTokenSupply" \
}

getVirtualEgldReserve() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getVirtualEgldReserve" \
}

getRewardsReserve() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getRewardsReserve" \
}

getDelegationAddressesList() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getDelegationAddressesList" \
}

###PARAMS
#1 - Address
getDelegationContractData() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --arguments $1 \
        --function="getDelegationContractData" \
}

getDelegationStatus() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getDelegationStatus" \
}

getDelegationClaimStatus() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getDelegationClaimStatus" \
}

###PARAMS
#1 - Token nonce
getRemainingEpochsUntilUnbond() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --arguments $1 \
        --function="getRemainingEpochsUntilUnbond" \
}

getRemainingEpochsUntilUnbondEndpoint() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --arguments $1 \
        --function="getRemainingEpochsUntilUnbond" \
        --send || return
}
