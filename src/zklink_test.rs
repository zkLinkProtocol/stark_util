use async_trait::async_trait;

use crate::client::StarkClient;
use crate::from_slice;
use crate::proto::*;
use crate::u256::U256;
use crate::zklink::ZkLink;

pub struct Test<'a> {
    client: &'a StarkClient,
}

impl<'a> From<&'a StarkClient> for Test<'a> {
    fn from(c: &'a StarkClient) -> Self {
        Test { client: c }
    }
}

#[async_trait]
impl<'a> ZkLink for Test<'a> {
    async fn stored_block_info_test(
        &self,
        _blocks_data: Vec<StoredBlockInfo>,
        i: usize,
    ) -> anyhow::Result<u64> {
        unimplemented!("method not allowed")
    }

    async fn commit_block_info_test(
        &self,
        _blocks_data: Vec<CommitBlockInfo>,
        i: usize,
        j: usize,
    ) -> anyhow::Result<usize> {
        unimplemented!("method not allowed")
    }

    async fn compressed_block_extra_info_test(
        &self,
        _blocks_extra_data: Vec<CompressedBlockExtraInfo>,
        i: usize,
        j: usize,
    ) -> anyhow::Result<U256> {
        unimplemented!("method not allowed")
    }

    async fn execute_block_info_test(
        &self,
        _blocks_data: Vec<ExecuteBlockInfo>,
        i: usize,
        j: usize,
        _op_type: u8,
    ) -> anyhow::Result<u8> {
        unimplemented!("method not allowed")
    }

    async fn u256test(&self, _u256: U256) -> anyhow::Result<(u128, u128)> {
        let ret = self.client.call("u256Test", _u256).await?;
        Ok(from_slice::<(u128, u128)>(ret.as_slice())?)
    }

    async fn u256s_test(&self, _u256s: Vec<U256>, i: usize) -> anyhow::Result<(u128, u128)> {
        let ret = self.client.call("u256sTest", (_u256s, i)).await?;
        Ok(from_slice::<(u128, u128)>(ret.as_slice())?)
    }

    async fn u8s_test1(&self, _u8s: Vec<u8>) -> anyhow::Result<usize> {
        let ret = self.client.call("u8sTest1", _u8s).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn u8s_test2(&self, _u8s: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let ret = self.client.call("u8sTest2", _u8s).await?;
        Ok(from_slice(ret.as_slice())?)
    }
}
