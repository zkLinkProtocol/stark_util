use anyhow::Result;
use async_trait::async_trait;

use super::model::*;
use crate::{contract::ContractCaller, primitive::U256};

#[async_trait]
pub trait ZkLinkTest {
    // =============Test Interface=============
    // Only test
    async fn stored_block_info_test(&self, blocks_data: Vec<StoredBlockInfo>, i: usize) -> Result<u64>;

    async fn commit_block_info_test(&self, blocks_data: Vec<CommitBlockInfo>, i: usize, j: usize) -> Result<usize>;

    async fn compressed_block_extra_info_test(&self,
                                              blocks_extra_data: Vec<CompressedBlockExtraInfo>,
                                              i: usize,
                                              j: usize)
                                              -> Result<U256>;

    async fn execute_block_info_test(&self,
                                     blocks_data: Vec<ExecuteBlockInfo>,
                                     i: usize,
                                     j: usize,
                                     op_type: u8)
                                     -> Result<u8>;

    async fn u256test(&self, u256: U256) -> Result<(u128, u128)>;

    async fn u256s_test(&self, u256s: Vec<U256>, i: usize) -> Result<(u128, u128)>;

    async fn u8s_test1(&self, u8s: Vec<u8>) -> Result<usize>;

    async fn u8s_test2(&self, u8s: Vec<u8>) -> Result<Vec<u8>>;
}

#[async_trait]
impl<T> ZkLinkTest for T
    where T: ContractCaller + Sync,
          <<T as ContractCaller>::Provider as starknet::providers::Provider>::Error: 'static
{
    async fn stored_block_info_test(&self, blocks_data: Vec<StoredBlockInfo>, i: usize) -> Result<u64> {
        self.call("StoredBlockInfoTest", (blocks_data, i)).await
    }

    async fn commit_block_info_test(&self, blocks_data: Vec<CommitBlockInfo>, i: usize, j: usize) -> Result<usize> {
        self.call("CommitBlockInfoTest", (blocks_data, i, j)).await
    }

    async fn compressed_block_extra_info_test(&self,
                                              blocks_extra_data: Vec<CompressedBlockExtraInfo>,
                                              i: usize,
                                              j: usize)
                                              -> Result<U256> {
        self.call("CompressedBlockExtraInfoTest", (blocks_extra_data, i, j)).await
    }

    async fn execute_block_info_test(&self,
                                     blocks_data: Vec<ExecuteBlockInfo>,
                                     i: usize,
                                     j: usize,
                                     op_type: u8)
                                     -> Result<u8> {
        self.call("ExecuteBlockInfoTest", (blocks_data, i, j, op_type)).await
    }

    async fn u256test(&self, u256: U256) -> Result<(u128, u128)> {
        self.call("u256Test", u256).await
    }

    async fn u256s_test(&self, u256s: Vec<U256>, i: usize) -> Result<(u128, u128)> {
        self.call("u256sTest", (u256s, i)).await
    }

    async fn u8s_test1(&self, u8s: Vec<u8>) -> Result<usize> {
        self.call("u8sTest1", u8s).await
    }

    async fn u8s_test2(&self, u8s: Vec<u8>) -> Result<Vec<u8>> {
        self.call("u8sTest2", u8s).await
    }
}

#[cfg(test)]
mod test {
    use starknet::accounts::ConnectedAccount;

    #[allow(unused_imports)]
    use crate::{
        builder::Builder,
        contract::ContractInstance,
        primitive::*,
        provider::*,
        zklink::model::{
            Bytes, CommitBlockInfo, CompressedBlockExtraInfo, ExecuteBlockInfo, OnchainOperationData, StoredBlockInfo,
        },
        zklink::zklink_test::ZkLinkTest,
    };

    fn contract() -> ContractInstance<impl ConnectedAccount> {
        #[allow(unused_variables)]
        let contract_clash_hash = "0x750c5cb7ba676049a2f8b7caabaf1d9dd4adbe7aa716735edde5aaa2e3d02b4";
        let private_key_hex = "0x029d821d79d49716c0760c79a3258f25c84875476cd7db2afce1856162715976";
        let account_address = "0x5686c52b6f38639eb9cfb3dfff1b3260315099aa045fcc0b4a865068ba36aad";
        let contract_address = "0xe402c3433801b22d90e257b1ebcdee7532b99d03fe14559d7db87185d7f794";
        let builder = || {
            Builder::new().set_contract_address(contract_address)?
                          .set_owner_address(account_address)?
                          .set_private_key(private_key_hex)?
                          .build()
        };
        builder().unwrap()
    }

    #[tokio::test]
    async fn test_u256s_test() {
        let r = contract().u256s_test(vec![PrimitiveU256::from(1u8).into()], 0).await;
        println!("{:?}", r);
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_u8_test() {
        let r = contract().u8s_test1(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).await;
        println!("{:?}", r);
        assert!(r.is_ok());

        let r = contract().u8s_test2(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).await;
        println!("{:?}", r);
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_stored_block_info() {
        let info = StoredBlockInfo { block_number: 1,
                                     priority_operations: 1,
                                     pending_onchain_operations_hash: PrimitiveU256::from(2).into(),
                                     timestamp: 3,
                                     state_hash: PrimitiveU256::from(4u8).into(),
                                     commitment: PrimitiveU256::from(5u8).into(),
                                     sync_hash: PrimitiveU256::from(6u8).into() };
        let r = contract().stored_block_info_test(vec![info], 0).await;
        println!("{r:?}");
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_compressed_block_extra_info() {
        let info = CompressedBlockExtraInfo { public_data_hash: PrimitiveU256::from(1u8).into(),
                                              offset_commitment_hash: PrimitiveU256::from(2u8).into(),
                                              onchain_operation_pubdata_hashs: vec![PrimitiveU256::from(1u8).into(),
                                                                                    PrimitiveU256::from(2u8).into(),] };
        let r = contract().compressed_block_extra_info_test(vec![info], 0, 0).await;
        println!("{:?}", r);
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_commit_block_info() {
        let op = OnchainOperationData { eth_witness: Bytes::default(), public_data_offset: 2 };
        let info = CommitBlockInfo { new_state_hash: PrimitiveU256::from(1u8).into(),
                                     public_data: Bytes::default(),
                                     timestamp: 2,
                                     onchain_operations: vec![op],
                                     block_number: 3,
                                     fee_account: 4 };
        println!("{info:?}");
        let r = contract().commit_block_info_test(vec![info], 0, 0).await;
        println!("{:?}", r);
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_execute_block_info() {
        let stored_block_info = StoredBlockInfo { block_number: 1,
                                                  priority_operations: 2,
                                                  pending_onchain_operations_hash: PrimitiveU256::from(3u8).into(),
                                                  timestamp: 4,
                                                  state_hash: PrimitiveU256::from(5u8).into(),
                                                  commitment: PrimitiveU256::from(6u8).into(),
                                                  sync_hash: PrimitiveU256::from(7u8).into() };

        let r = contract().stored_block_info_test(vec![stored_block_info.clone()], 0).await;
        println!("{:?}", r);
        assert!(r.is_ok());

        // TODO Fix bug
        // let info = ExecuteBlockInfo { stored_block_info, pending_onchain_ops_pubdata: vec![Bytes { size: 2, data: vec![1, 2] }] };
        // let r = contract().execute_block_info_test(vec![info], 0, 0, 1).await;
        // println!("{r:?}");
        // assert!(r.is_ok());
    }
}
