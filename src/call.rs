use serde::Serialize;
use starknet::accounts::ConnectedAccount;
use starknet::core::types::{BlockId, BlockTag, FunctionCall};
use starknet::core::utils::get_selector_from_name;
use starknet::providers::Provider;

use crate::client::StarkClient;
use crate::primitive::FieldElement;
use crate::to_field_elements;

pub struct Call<'a> {
    client: &'a StarkClient,
}

impl<'a> From<&'a StarkClient> for Call<'a> {
    fn from(client: &'a StarkClient) -> Self {
        Call { client }
    }
}

impl<'a> Call<'a> {
    pub async fn call<T>(
        &self,
        contract_address: FieldElement,
        func_name: &str,
        calldata: T,
    ) -> anyhow::Result<Vec<FieldElement>>
    where
        T: Serialize,
    {
        Ok(self
            .client
            .owner()
            .provider()
            .call(
                FunctionCall {
                    contract_address,
                    entry_point_selector: get_selector_from_name(func_name)?,
                    calldata: to_field_elements(calldata)?,
                },
                BlockId::Tag(BlockTag::Pending),
            )
            .await?)
    }
}
