use crate::{decoder::Decode, FieldElement};
use async_trait::async_trait;

pub trait Contract<O> {
    fn contract(&self) -> O;
}

#[async_trait]
pub trait Callable {
    async fn call<T: Decode, O: Decode>(&self, contract_address: FieldElement, func_name: &str, calldata: T) -> anyhow::Result<O>;
}
