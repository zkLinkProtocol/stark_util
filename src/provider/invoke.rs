#![allow(dead_code)]

use serde::Serialize;
use starknet::{
    accounts::{Account, Call},
    core::utils::get_selector_from_name,
};

use crate::{
    primitive::{FieldElement, TxHash},
    provider::StarkClient,
    to_field_elements,
};

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
        where T: Serialize {
        let result =
            self.client
                .owner()
                .execute(vec![Call { to: contract_address, selector: get_selector_from_name(func_name)?, calldata: to_field_elements(calldata)? }])
                .send()
                .await?;
        Ok(result.transaction_hash.into())
    }
}
