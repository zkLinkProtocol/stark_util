use async_trait::async_trait;
// use starknet_api::contract_address;

use crate::{contract::Callable, from_slice, proto::*, u256::U256, zklink::ZkLink};

#[async_trait]
impl<T: Callable + Sync> ZkLink for T {
    async fn stored_block_info_test(&self, blocks_data: Vec<StoredBlockInfo>, i: usize) -> anyhow::Result<u64> {
        let ret = self.call("StoredBlockInfoTest", (blocks_data, i)).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn commit_block_info_test(&self, blocks_data: Vec<CommitBlockInfo>, i: usize, j: usize) -> anyhow::Result<usize> {
        let ret = self.call("CommitBlockInfoTest", (blocks_data, i, j)).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn compressed_block_extra_info_test(&self, blocks_extra_data: Vec<CompressedBlockExtraInfo>, i: usize, j: usize) -> anyhow::Result<U256> {
        let ret = self.call("CompressedBlockExtraInfoTest", (blocks_extra_data, i, j)).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn execute_block_info_test(&self, blocks_data: Vec<ExecuteBlockInfo>, i: usize, j: usize, op_type: u8) -> anyhow::Result<u8> {
        let ret = self.call("ExecuteBlockInfoTest", (blocks_data, i, j, op_type)).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn u256test(&self, u256: U256) -> anyhow::Result<(u128, u128)> {
        let ret = self.call("u256Test", u256).await?;
        Ok(from_slice::<(u128, u128)>(ret.as_slice())?)
    }

    async fn u256s_test(&self, u256s: Vec<U256>, i: usize) -> anyhow::Result<(u128, u128)> {
        let ret = self.call("u256sTest", (u256s, i)).await?;
        Ok(from_slice::<(u128, u128)>(ret.as_slice())?)
    }

    async fn u8s_test1(&self, u8s: Vec<u8>) -> anyhow::Result<usize> {
        let ret = self.call("u8sTest1", u8s).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn u8s_test2(&self, u8s: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let ret = self.call("u8sTest2", u8s).await?;
        Ok(from_slice(ret.as_slice())?)
    }
}
