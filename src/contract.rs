use async_trait::async_trait;
use serde::Serialize;

use crate::{
    primitive::{FieldElement, TxHash},
    provider::StarkClient,
};

#[async_trait]
pub trait Callable {
    async fn invoke<T: Serialize + Send>(&self, func_name: &str, calldata: T) -> anyhow::Result<TxHash>;
    async fn call<T: Serialize + Send>(&self, func_name: &str, calldata: T) -> anyhow::Result<Vec<FieldElement>>;
}

pub struct Contract {
    client: StarkClient,
    address: FieldElement,
}

impl Contract {
    pub fn new(client: StarkClient, address: FieldElement) -> Self {
        Contract { client, address }
    }
}

#[async_trait]
impl Callable for Contract {
    async fn invoke<T: Serialize + Send>(&self, func_name: &str, calldata: T) -> anyhow::Result<TxHash> {
        self.client.invoke(self.address, func_name, calldata).await
    }

    async fn call<T: Serialize + Send>(&self, func_name: &str, calldata: T) -> anyhow::Result<Vec<FieldElement>> {
        self.client.call(self.address, func_name, calldata).await
    }
}
