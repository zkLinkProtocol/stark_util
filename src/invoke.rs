use serde::Serialize;
use starknet::{
    accounts::{Account, Call, ConnectedAccount, SingleOwnerAccount},
    core::{
        types::{BlockId, BlockTag, FunctionCall},
        utils::get_selector_from_name,
    },
    providers::Provider,
};

use crate::{client::StarkClient, primitive::FieldElement, proto::TxHash, to_field_elements};

// TODO
pub struct Invoke<'a> {
    // owner
    client: &'a StarkClient,
}

impl<'a> From<&'a StarkClient> for Invoke<'a> {
    fn from(client: &'a StarkClient) -> Self {
        Invoke { client }
    }
}

impl<'a> Invoke<'a> {
    pub async fn invoke<T>(&self, contract_address: FieldElement, func_name: &str, calldata: T) -> anyhow::Result<TxHash>
        where T: Serialize
    {
        let result = self.client
                         .owner()
                         .execute(vec![Call { to: contract_address,
                                              selector: get_selector_from_name(func_name)?,
                                              calldata: to_field_elements(calldata)? }])
                         .send()
                         .await?;
        Ok(result.transaction_hash.into())
    }
}
