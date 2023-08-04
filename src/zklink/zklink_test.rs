use async_trait::async_trait;
// use starknet_api::contract_address;
use anyhow::Result;

use crate::{contract::Callable, from_slice, primitive::U256};
use super::model::*;

#[async_trait]
pub trait ZkLinkTest {
    // =============Test Interface=============
    // Only test
    async fn stored_block_info_test(&self, blocks_data: Vec<StoredBlockInfo>, i: usize) -> Result<u64>;

    async fn commit_block_info_test(&self, blocks_data: Vec<CommitBlockInfo>, i: usize, j: usize) -> Result<usize>;

    async fn compressed_block_extra_info_test(&self, blocks_extra_data: Vec<CompressedBlockExtraInfo>, i: usize, j: usize) -> Result<U256>;

    async fn execute_block_info_test(&self, blocks_data: Vec<ExecuteBlockInfo>, i: usize, j: usize, op_type: u8) -> Result<u8>;

    async fn u256test(&self, u256: U256) -> Result<(u128, u128)>;

    async fn u256s_test(&self, u256s: Vec<U256>, i: usize) -> Result<(u128, u128)>;

    async fn u8s_test1(&self, u8s: Vec<u8>) -> Result<usize>;

    async fn u8s_test2(&self, u8s: Vec<u8>) -> Result<Vec<u8>>;
}

#[async_trait]
impl<T: Callable + Sync> ZkLinkTest for T {
    async fn stored_block_info_test(&self, blocks_data: Vec<StoredBlockInfo>, i: usize) -> Result<u64> {
        let ret = self.call("StoredBlockInfoTest", (blocks_data, i)).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn commit_block_info_test(&self, blocks_data: Vec<CommitBlockInfo>, i: usize, j: usize) -> Result<usize> {
        let ret = self.call("CommitBlockInfoTest", (blocks_data, i, j)).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn compressed_block_extra_info_test(&self, blocks_extra_data: Vec<CompressedBlockExtraInfo>, i: usize, j: usize) -> Result<U256> {
        let ret = self.call("CompressedBlockExtraInfoTest", (blocks_extra_data, i, j)).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn execute_block_info_test(&self, blocks_data: Vec<ExecuteBlockInfo>, i: usize, j: usize, op_type: u8) -> Result<u8> {
        let ret = self.call("ExecuteBlockInfoTest", (blocks_data, i, j, op_type)).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn u256test(&self, u256: U256) -> Result<(u128, u128)> {
        let ret = self.call("u256Test", u256).await?;
        Ok(from_slice::<(u128, u128)>(ret.as_slice())?)
    }

    async fn u256s_test(&self, u256s: Vec<U256>, i: usize) -> Result<(u128, u128)> {
        let ret = self.call("u256sTest", (u256s, i)).await?;
        Ok(from_slice::<(u128, u128)>(ret.as_slice())?)
    }

    async fn u8s_test1(&self, u8s: Vec<u8>) -> Result<usize> {
        let ret = self.call("u8sTest1", u8s).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn u8s_test2(&self, u8s: Vec<u8>) -> Result<Vec<u8>> {
        let ret = self.call("u8sTest2", u8s).await?;
        Ok(from_slice(ret.as_slice())?)
    }
}

// #[cfg(test)]
// mod test {
//     use crate::{
//         builder::Builder,
//         client::StarkClient,
//         network::Network,
//         proto::{Bytes, CommitBlockInfo, CompressedBlockExtraInfo, ExecuteBlockInfo, OnchainOperationData, StoredBlockInfo, TxHash},
//         u256::PrimitiveU256,
//         U256,
//     };
//
//     fn client() -> StarkClient {
//         let web3_url = "http://127.0.0.1:5050";
//         let private_key_hex = "6fb84183efc4de5a4707ac7ad487d5e1db4ec34a2c1500ee25fe6ab29940462";
//         let address = "0x13528b84b5a4ed4a7aff3b3a27363565f38608499f1404f73e15c11fce9aa5d";
//         let contract_address = "0x474c2b5858139a7d7f20e71f836fc98f130c2c2992888433fbdce742a95d564";
//         let network = Network::Goerli1;
//         use crate::builder::Builder;
//         let client = Builder::new().set_network(network)?.set_url(web3_url)?.set_owner_address(address)?.set_private_key(private_key_hex)?.build();
//         client.unwrap()
//     }
//
//     #[tokio::test]
//     async fn test_u128_list() {
//         let r = client().test_u128_list(vec![1u128], 0).await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_u256_list() {
//         let r = client().test_u256_list(vec![PrimitiveU256::from(1u8).into()], 0).await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_u8_array() {
//         let r = client().test_u8_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_bytes_list() {
//         let b1 = Bytes { size: 1, data: vec![1, 2, 3] };
//         let b2 = Bytes::default();
//         let r = client().test_bytes_list(vec![b1, b2], 1).await;
//         println!("{r:?}");
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_stored_block_info() {
//         let info = StoredBlockInfo { block_number: 1,
//                                      priority_operations: 1,
//                                      pending_onchain_operations_hash: PrimitiveU256::from(2).into(),
//                                      timestamp: 3,
//                                      state_hash: PrimitiveU256::from(4u8).into(),
//                                      commitment: PrimitiveU256::from(5u8).into(),
//                                      sync_hash: PrimitiveU256::from(6u8).into() };
//         let r = client().test_stored_block_info(vec![info], 0).await;
//         println!("{r:?}");
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_compressed_block_extra_info() {
//         let info = CompressedBlockExtraInfo { public_data_hash: PrimitiveU256::from(1u8).into(),
//                                               offset_commitment_hash: PrimitiveU256::from(2u8).into(),
//                                               onchain_operation_pubdata_hashs: vec![PrimitiveU256::from(1u8).into(),
//                                                                                     PrimitiveU256::from(2u8).into(),] };
//         let r = client().test_compressed_block_extra_info(vec![info], 0, 0).await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_commit_block_info() {
//         let op = OnchainOperationData { eth_witness: Bytes::default(), public_data_offset: 2 };
//         let info = CommitBlockInfo { new_state_hash: PrimitiveU256::from(1u8).into(),
//                                      public_data: Bytes::default(),
//                                      timestamp: 2,
//                                      onchain_operations: vec![op],
//                                      block_number: 3,
//                                      fee_account: 4 };
//         println!("{info:?}");
//         let r = client().test_commit_block_info(vec![info], 0, 0).await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_execute_block_info() {
//         let stored_block_info = StoredBlockInfo { block_number: 1,
//                                                   priority_operations: 2,
//                                                   pending_onchain_operations_hash: PrimitiveU256::from(3u8).into(),
//                                                   timestamp: 4,
//                                                   state_hash: PrimitiveU256::from(5u8).into(),
//                                                   commitment: PrimitiveU256::from(6u8).into(),
//                                                   sync_hash: PrimitiveU256::from(7u8).into() };
//
//         let r = client().test_stored_block_info(vec![stored_block_info.clone()], 0).await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//
//         let info = ExecuteBlockInfo { stored_block_info, pending_onchain_ops_pubdata: vec![Bytes { size: 2, data: vec![1, 2] }] };
//         let r = client().test_execute_block_info2(vec![info.clone()], 0, 0, 1).await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//         let r = client().test_execute_block_info(vec![info], 0, 0, 1).await;
//         println!("{r:?}");
//         assert!(r.is_ok());
//     }
// }
