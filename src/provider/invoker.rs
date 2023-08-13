use async_trait::async_trait;
use serde::Serialize;
use starknet::{
    accounts::{Call, ConnectedAccount},
    core::utils::get_selector_from_name,
};

use crate::{
    primitive::{FieldElement, TxHash},
    to_field_elements,
};

#[async_trait]
pub trait Invoker {
    async fn invoker<I: Serialize + Send>(&self,
                                          contract_address: FieldElement,
                                          func_name: &str,
                                          calldata: I)
                                          -> anyhow::Result<TxHash>;
}

#[async_trait]
impl<T: ConnectedAccount + Send + Sync + 'static> Invoker for T {
    async fn invoker<I>(&self, contract_address: FieldElement, func_name: &str, calldata: I) -> anyhow::Result<TxHash>
        where I: Serialize + Send
    {
        let result = self.execute(vec![Call { to: contract_address,
                                              selector: get_selector_from_name(func_name)?,
                                              calldata: to_field_elements(calldata)? }])
                         .send()
                         .await?;
        Ok(result.transaction_hash.into())
    }
}
