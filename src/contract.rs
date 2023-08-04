use async_trait::async_trait;
use serde::Serialize;

use crate::{client::StarkClient, proto::TxHash, FieldElement};

// pub trait Contract<O> {
//     fn contract(&self) -> O;
// }

// impl<'a, T, O> Contract<O> for &'a T where &'a T: Into<O>
// {
//     fn contract(&self) -> O {
//         (*self).into()
//     }
// }

// #[async_trait]
// pub trait Callable {
//     async fn call<T: Decode, O: Decode>(&self, contract_address:
// FieldElement, func_name: &str, calldata: T) -> anyhow::Result<O>; }

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
