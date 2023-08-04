use serde::Serialize;
use starknet::{
    accounts::{Account, Call, ConnectedAccount, SingleOwnerAccount},
    core::{
        types::{BlockId, BlockTag, FieldElement, FunctionCall},
        utils::get_selector_from_name,
    },
    providers::Provider,
    signers::LocalWallet,
};

use crate::{proto::TxHash, provider::ExtendedProvider, to_field_elements};

pub struct StarkClient {
    owner: SingleOwnerAccount<ExtendedProvider, LocalWallet>,
    #[allow(dead_code)]
    address: FieldElement, //TODO remove user address
}

impl StarkClient {
    pub fn new(owner: SingleOwnerAccount<ExtendedProvider, LocalWallet>, address: FieldElement) -> Self {
        Self { owner, address }
    }

    pub fn owner(&self) -> &SingleOwnerAccount<ExtendedProvider, LocalWallet> {
        &self.owner
    }

    pub async fn get_last_block_number(&self) -> anyhow::Result<u64> {
        Ok(self.owner().provider().block_number().await?)
    }

    pub async fn get_pending_nonce(&self, address: FieldElement) -> anyhow::Result<FieldElement> {
        Ok(self.owner().provider().get_nonce(BlockId::Tag(BlockTag::Pending), address).await?)
    }

    // TODO
    pub async fn invoke<T>(&self, contract_address: FieldElement, func_name: &str, calldata: T) -> anyhow::Result<TxHash>
        where T: Serialize {
        let selector = get_selector_from_name(func_name).unwrap();
        let calldata_elements = to_field_elements(calldata)?;
        let result = self.owner().execute(vec![Call { to: contract_address, selector, calldata: calldata_elements }]).send().await?;
        let tx_hash = result.transaction_hash;
        Ok(tx_hash.into())
    }

    // TODO
    pub async fn call<T>(&self, contract_address: FieldElement, func_name: &str, calldata: T) -> anyhow::Result<Vec<FieldElement>>
        where T: Serialize {
        let selector = get_selector_from_name(func_name)?;
        let calldata = to_field_elements(calldata)?;
        Ok(self.owner()
               .provider()
               .call(FunctionCall { contract_address, entry_point_selector: selector, calldata }, BlockId::Tag(BlockTag::Pending))
               .await?)
    }
}

// #[cfg(test)]
// mod test {
//     use crate::builder::Builder;
//     use crate::client::StarkClient;
//     use crate::network::Network;
//     use crate::proto::{
//         Bytes, CommitBlockInfo, CompressedBlockExtraInfo, ExecuteBlockInfo,
// OnchainOperationData,         StoredBlockInfo, TxHash,
//     };
//     use crate::u256::PrimitiveU256;
//     use crate::U256;
//
//     impl StarkClient {
//         pub async fn test_u128_list(
//             &self,
//             calldata: Vec<u128>,
//             index: usize,
//         ) -> anyhow::Result<TxHash> { self.invoke("u128Test", (calldata,
//           index)).await
//         }
//         pub async fn test_u256_list(
//             &self,
//             calldata: Vec<U256>,
//             index: usize,
//         ) -> anyhow::Result<TxHash> { self.invoke("u256s_test", (calldata,
//           index)).await
//         }
//
//         pub async fn test_u8_array(&self, calldata: Vec<u8>) ->
// anyhow::Result<TxHash> {             self.invoke("u8s_test1", calldata).await
//         }
//
//         pub async fn test_bytes_list(
//             &self,
//             calldata: Vec<Bytes>,
//             index: usize,
//         ) -> anyhow::Result<TxHash> { self.invoke("bytesListTest", (calldata,
//           index)).await
//         }
//
//         pub async fn test_stored_block_info(
//             &self,
//             info_list: Vec<StoredBlockInfo>,
//             i: usize,
//         ) -> anyhow::Result<TxHash> { self.invoke("stored_block_info_test",
//           (info_list, i)).await
//         }
//
//         pub async fn test_commit_block_info(
//             &self,
//             info_list: Vec<CommitBlockInfo>,
//             i: usize,
//             j: usize,
//         ) -> anyhow::Result<TxHash> { self.invoke("commit_block_info_test",
//           (info_list, i, j)) .await
//         }
//
//         pub async fn test_compressed_block_extra_info(
//             &self,
//             info_list: Vec<CompressedBlockExtraInfo>,
//             i: usize,
//             j: usize,
//         ) -> anyhow::Result<TxHash> {
//           self.invoke("compressed_block_extra_info_test", (info_list, i, j))
//           .await
//         }
//
//         pub async fn test_execute_block_info(
//             &self,
//             info_list: Vec<ExecuteBlockInfo>,
//             i: usize,
//             j: u8,
//             op_type: u8,
//         ) -> anyhow::Result<TxHash> { self.invoke("execute_block_info_test",
//           (info_list, i, j, op_type)) .await
//         }
//
//         pub async fn test_execute_block_info2(
//             &self,
//             info_list: Vec<ExecuteBlockInfo>,
//             i: usize,
//             j: u8,
//             op_type: u8,
//         ) -> anyhow::Result<TxHash> { self.invoke("ExecuteBlockInfoTest2",
//           (info_list, i, j, op_type)) .await
//         }
//     }
//
//     fn client() -> StarkClient {
//         let web3_url = "http://127.0.0.1:5050";
//         let private_key_hex =
// "6fb84183efc4de5a4707ac7ad487d5e1db4ec34a2c1500ee25fe6ab29940462";
//         let address =
// "0x13528b84b5a4ed4a7aff3b3a27363565f38608499f1404f73e15c11fce9aa5d";
//         let contract_address =
// "0x474c2b5858139a7d7f20e71f836fc98f130c2c2992888433fbdce742a95d564";
// let network = Network::Goerli1;         use crate::builder::Builder;
//         let client = Builder::new()
//             .set_network(network)?
//             .set_url(web3_url)?
//             .set_owner_address(address)?
//             .set_private_key(private_key_hex)?
//             .build();
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
//         let r = client()
//             .test_u256_list(vec![PrimitiveU256::from(1u8).into()], 0)
//             .await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_u8_array() {
//         let r = client()
//             .test_u8_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
//             .await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_bytes_list() {
//         let b1 = Bytes {
//             size: 1,
//             data: vec![1, 2, 3],
//         };
//         let b2 = Bytes::default();
//         let r = client().test_bytes_list(vec![b1, b2], 1).await;
//         println!("{r:?}");
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_stored_block_info() {
//         let info = StoredBlockInfo {
//             block_number: 1,
//             priority_operations: 1,
//             pending_onchain_operations_hash: PrimitiveU256::from(2).into(),
//             timestamp: 3,
//             state_hash: PrimitiveU256::from(4u8).into(),
//             commitment: PrimitiveU256::from(5u8).into(),
//             sync_hash: PrimitiveU256::from(6u8).into(),
//         };
//         let r = client().test_stored_block_info(vec![info], 0).await;
//         println!("{r:?}");
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_compressed_block_extra_info() {
//         let info = CompressedBlockExtraInfo {
//             public_data_hash: PrimitiveU256::from(1u8).into(),
//             offset_commitment_hash: PrimitiveU256::from(2u8).into(),
//             onchain_operation_pubdata_hashs: vec![
//                 PrimitiveU256::from(1u8).into(),
//                 PrimitiveU256::from(2u8).into(),
//             ],
//         };
//         let r = client()
//             .test_compressed_block_extra_info(vec![info], 0, 0)
//             .await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_commit_block_info() {
//         let op = OnchainOperationData {
//             eth_witness: Bytes::default(),
//             public_data_offset: 2,
//         };
//         let info = CommitBlockInfo {
//             new_state_hash: PrimitiveU256::from(1u8).into(),
//             public_data: Bytes::default(),
//             timestamp: 2,
//             onchain_operations: vec![op],
//             block_number: 3,
//             fee_account: 4,
//         };
//         println!("{info:?}");
//         let r = client().test_commit_block_info(vec![info], 0, 0).await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_execute_block_info() {
//         let stored_block_info = StoredBlockInfo {
//             block_number: 1,
//             priority_operations: 2,
//             pending_onchain_operations_hash: PrimitiveU256::from(3u8).into(),
//             timestamp: 4,
//             state_hash: PrimitiveU256::from(5u8).into(),
//             commitment: PrimitiveU256::from(6u8).into(),
//             sync_hash: PrimitiveU256::from(7u8).into(),
//         };
//
//         let r = client()
//             .test_stored_block_info(vec![stored_block_info.clone()], 0)
//             .await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//
//         let info = ExecuteBlockInfo {
//             stored_block_info,
//             pending_onchain_ops_pubdata: vec![Bytes {
//                 size: 2,
//                 data: vec![1, 2],
//             }],
//         };
//         let r = client()
//             .test_execute_block_info2(vec![info.clone()], 0, 0, 1)
//             .await;
//         println!("{:?}", r);
//         assert!(r.is_ok());
//         let r = client().test_execute_block_info(vec![info], 0, 0, 1).await;
//         println!("{r:?}");
//         assert!(r.is_ok());
//     }
// }
