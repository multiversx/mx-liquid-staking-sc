WALLET_PEM="/Users/sorinpetreasca/DevKitt/walletKey.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"

LIQUID_STAKING_WASM_PATH="/Users/sorinpetreasca/Elrond/sc-liquid-staking-rs/liquid-staking/output/liquid-staking.wasm"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqqgm8nl92lzq5gwzscav8ugya7rr0yukx5dsqh7r8av"

deploySC() {
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${LIQUID_STAKING_WASM_PATH} \
        --pem=${WALLET_PEM} \
        --gas-limit=100000000 \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

upgradeSC() {
    erdpy --verbose contract upgrade ${CONTRACT_ADDRESS} --recall-nonce \
        --bytecode=${LIQUID_STAKING_WASM_PATH} \
        --pem=${WALLET_PEM} \
        --gas-limit=100000000 \
        --metadata-payable \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

TOKEN_NAME=0x4c53544f4b454e #LSTOKEN
TOKEN_TICKER=0x4c5354 #LST
TOKEN_DECIMALS=18
registerLsToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
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
        --gas-limit=60000000 \
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
DELEGATION_ADDRESS="erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqyhllllsv4k7x2"
TOTAL_STAKED=1250
DELEGATION_CAP=1000000 
NR_NODES=2
APY=10
whitelistDelegationContract() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="whitelistDelegationContract" \
        --arguments ${DELEGATION_ADDRESS} ${TOTAL_STAKED} ${DELEGATION_CAP} ${NR_NODES} ${APY}\
        --send || return
}

changeDelegationContractParams() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="changeDelegationContractParams" \
        --arguments ${DELEGATION_ADDRESS} ${TOTAL_STAKED} ${DELEGATION_CAP} ${NR_NODES} ${APY}\
        --send || return
}

###PARAMS
#1 - Amount
addLiquidity() {
        erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --value=$1 \
        --function="addLiquidity" \
        --send || return
}

###PARAMS
#1 - Amount
LS_TOKEN=0x4c53544f4b454e
DEPOSIT_METHOD="removeLiquidity" 
removeLiquidity() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="ESDTTransfer" \
        --arguments ${LS_TOKEN} $1 ${DEPOSIT_METHOD} \
        --send || return
}

###PARAMS
#1 - Amount
LS_UNSTAKE_TOKEN=0x4c53554e5354414b45
DEPOSIT_METHOD="unbondTokens" 
unbondTokens() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="ESDTTransfer" \
        --arguments ${LS_UNSTAKE_TOKEN} $1 ${DEPOSIT_METHOD} \
        --send || return
}

claimRewards() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=600000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="claimRewards" \
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

getRewardsReserve() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getRewardsReserve" \
}
