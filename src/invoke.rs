use serde::Serialize;
use starknet::accounts::ConnectedAccount;
use starknet::accounts::{Account, Call, SingleOwnerAccount};
use starknet::core::types::{BlockId, BlockTag, FunctionCall};
use starknet::core::utils::get_selector_from_name;
use starknet::providers::Provider;

use crate::client::StarkClient;
use crate::primitive::FieldElement;
use crate::proto::TxHash;
use crate::to_field_elements;

pub struct Invoke<'a> {
    client: &'a StarkClient,
}

impl<'a> From<&'a StarkClient> for Invoke<'a> {
    fn from(client: &'a StarkClient) -> Self {
        Invoke { client }
    }
}

impl<'a> Invoke<'a> {
    pub async fn invoke<T>(
        &self,
        contract_address: FieldElement,
        func_name: &str,
        calldata: T,
    ) -> anyhow::Result<TxHash>
    where
        T: Serialize,
    {
        let result = self
            .client
            .owner()
            .execute(vec![Call {
                to: contract_address,
                selector: get_selector_from_name(func_name)?,
                calldata: to_field_elements(calldata)?,
            }])
            .send()
            .await?;
        Ok(result.transaction_hash.into())
    }
}
