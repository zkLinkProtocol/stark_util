use serde::{Deserialize, Serialize};

use crate::{primitive::*, U256};

// TODO: change some struct to this type of Bytes
// TODO: rename Bytes
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Bytes {
    /// the number of bytes in the Bytes
    pub size: usize,
    /// the data of the Bytes
    pub data: Vec<u128>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TxHash(FieldElement);

impl From<TxHash> for String {
    fn from(value: TxHash) -> Self {
        let raw = value.0.to_bytes_be();
        let s = hex::encode(raw);
        let s = s.trim_start_matches('0');
        s.into()
    }
}

impl From<FieldElement> for TxHash {
    fn from(value: FieldElement) -> Self {
        Self(value)
    }
}

impl AsRef<FieldElement> for TxHash {
    fn as_ref(&self) -> &FieldElement {
        &self.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct StoredBlockInfo {
    /// Rollup block number
    pub block_number: u64,
    /// Number of priority operations processed
    pub priority_operations: u64,
    /// Hash of all operations that must be processed after verify
    pub pending_onchain_operations_hash: U256,
    /// Rollup block timestamp
    pub timestamp: u64,
    /// Root hash of the rollup state
    pub state_hash: U256,
    /// Verified input for the ZkLink circuit
    pub commitment: U256,
    /// Used for cross chain block verify
    pub sync_hash: U256,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CommitBlockInfo {
    pub new_state_hash: U256,
    pub public_data: Bytes,
    pub timestamp: u64,
    pub onchain_operations: Vec<OnchainOperationData>,
    pub block_number: u64,
    pub fee_account: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct OnchainOperationData {
    pub eth_witness: Bytes,
    pub public_data_offset: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CompressedBlockExtraInfo {
    /// pubdata hash of all chains
    pub public_data_hash: U256,
    /// all chains pubdata offset commitment hash
    pub offset_commitment_hash: U256,
    /// onchain operation pubdata hash of the all other chains
    pub onchain_operation_pubdata_hashs: Vec<U256>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ExecuteBlockInfo {
    /// the block info that will be executed
    pub stored_block_info: StoredBlockInfo,
    /// onchain ops(e.g. Withdraw, ForcedExit, FullExit) that will be executed
    pub pending_onchain_ops_pubdata: Vec<Bytes>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Deposit {
    // 1 byte, deposit from which chain that identified by l2 chain id
    pub chain_id: u8,
    // 4 bytes, the account id bound to the owner address, ignored at serialization and will be set when the block is submitted
    pub account_id: u32,
    // 1 byte, the sub account is bound to account, default value is 0(the global public sub account)
    pub sub_account_id: u8,
    // 2 bytes, the token that registered to l2
    pub token_id: u16,
    // 2 bytes, the token that user increased in l2
    pub target_token_id: u16,
    // 16 bytes, the token amount deposited to l2
    pub amount: u128,
    // 32 bytes, the address that receive deposited token at l2
    pub owner: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct FullExit {
    /// 1 byte, withdraw to which chain that identified by l2 chain id
    pub chain_id: u8,
    /// 4 bytes, the account id to withdraw from
    pub account_id: u32,
    /// 1 byte, the sub account is bound to account, default value is 0(the global public sub account)
    pub sub_account_id: u8,
    /// 32 bytes, the address that own the account at l2
    pub owner: String,
    /// 2 bytes, the token that withdraw to l1
    pub token_id: u16,
    /// 2 bytes, the token that deducted in l2
    pub src_token_id: u16,
    /// 16 bytes, the token amount that fully withdrawn to owner, ignored at serialization and will be set when the block is submitted
    pub amount: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Withdraw {
    /// 1 byte, which chain the withdraw happened
    pub chain_id: u8,
    /// 4 bytes, the account id to withdraw from
    pub account_id: u32,
    /// 2 bytes, the token that to withdraw
    pub token_id: u16,
    /// 16 bytes, the token amount to withdraw
    pub amount: u128,
    /// 32 bytes, the address to receive token
    pub owner: String,
    /// 4 bytes, zero means normal withdraw, not zero means fast withdraw and the value is the account nonce
    pub nonce: u32,
    /// 2 bytes, fast withdraw fee rate taken by accepter
    pub fast_withdraw_fee_rate: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ForcedExit {
    /// 1 byte, which chain the force exit happened
    pub chain_id: u8,
    /// 2 bytes, the token that to withdraw
    pub token_id: u16,
    /// 16 bytes, the token amount to withdraw
    pub amount: u128,
    /// 32 bytes, the address to receive token
    pub target: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ChangePubKey {
    /// 1 byte, which chain to verify(only one chain need to verify for gas saving)
    pub chain_id: u8,
    /// 4 bytes, the account that to change pubkey
    pub account_id: u32,
    /// 20 bytes, hash of the new rollup pubkey
    pub pub_key_hash: String,
    /// 32 bytes, the owner that own this account
    pub owner: String,
    /// 4 bytes, the account nonce
    pub nonce: u32,
}

// Token info stored in zkLink
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Token {
    pub token_id: u16,                  // token id defined by zkLink
    pub token_address: ContractAddress, // token address in l1
    pub decimals: u8,                   // token decimals in l1
    pub standard: bool,                 // if token a pure erc20 or not
}

// Recursive proof input data (individual commitments are constructed onchain)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ProofInput {
    pub recursive_input: Vec<U256>,
    pub proof: Vec<U256>,
    pub commitments: Vec<U256>,
    pub vk_indexes: Vec<u8>,
    pub subproofs_limbs: Vec<U256>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{from_slice, to_field_elements, u256::PrimitiveU256};

    #[test]
    fn test_tx_hash() {
        let tx_hash: TxHash = FieldElement::from(100u64).into();
        let s: String = tx_hash.into();
        println!("{s}");
    }

    #[test]
    fn test_serde_bytes() {
        let b = Bytes { size: 1,
                        data: vec![1, 2, 3] };
        let elements = to_field_elements(b.clone()).unwrap();
        assert_eq!(elements.len(), 5);
        let b2 = from_slice(&elements).unwrap();
        assert_eq!(b, b2);
    }

    #[test]
    fn test_serde_commit_block_info() {
        let data = OnchainOperationData { // 2 elements
                                          eth_witness: Bytes::default(),
                                          // 1 elements
                                          public_data_offset: 1 };
        let info = CommitBlockInfo { // 2 elements
                                     new_state_hash: PrimitiveU256::from(1u8).into(),
                                     // 2 elements
                                     public_data: Bytes::default(),
                                     // 1 element
                                     timestamp: 1,
                                     // 1 + 12 elements
                                     onchain_operations: vec![data],
                                     // 1 element
                                     block_number: 1,
                                     // 1 element
                                     fee_account: 1 };
        let field_elements = to_field_elements(info.clone()).unwrap();
        let info2 = from_slice(&field_elements).unwrap();
        assert_eq!(info, info2);
    }

    #[test]
    fn test_serde_stored_block_info() {
        let info = StoredBlockInfo { block_number: 1,
                                     priority_operations: 1,
                                     pending_onchain_operations_hash: PrimitiveU256::from(1).into(),
                                     timestamp: 1,
                                     state_hash: PrimitiveU256::from(1u8).into(),
                                     commitment: PrimitiveU256::from(2u8).into(),
                                     sync_hash: PrimitiveU256::from(3u8).into() };
        let field_elements = to_field_elements(info.clone()).unwrap();
        assert_eq!(field_elements.len(), 11);
        let info2 = from_slice(&field_elements).unwrap();
        assert_eq!(info, info2);
    }
}
