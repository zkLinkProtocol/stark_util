#![allow(dead_code)]

use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use stark_util::{
    contract::Callable,
    from_slice,
    primitive::{FieldElement, TxHash},
    Builder, Contract,
};

const PRIVATE_KEY: &str = "0x029d821d79d49716c0760c79a3258f25c84875476cd7db2afce1856162715976";

const PUBLIC_KEY: &str = "0x2abb6a1b2cd549fdf87835d1c04a8c156ac4d42441b5b7fcb1386768558a7be";
const ACCOUNT: &str = "0x5686c52b6f38639eb9cfb3dfff1b3260315099aa045fcc0b4a865068ba36aad";

const ACCOUNT1: &str = "0x0131159c04f780f71bd16c0f453f25f769d80878b8ddeffcccbe24211b9bbd62";
const ACCOUNT2: &str = "0x06ac7d3ef3458c6372e9f0dbb32c8bd023f0d5f7a98c650b105a5cc6dbde1cbc";
const ACCOUNT3: &str = "0x065f81fa8f222be104e463afc51bc97ea0d93d21e0bbfbfdca18c713c84a544f";
const COUNTER_CONTRACT_ADDRESS: &str = "0x0311bb7385271f9fa3754218f4bf097a784c308da898df405b84d571f5ed7468";

fn contract() -> Result<Contract> {
    let builder = Builder::new();
    builder.set_private_key(PRIVATE_KEY)?.set_owner_address(ACCOUNT)?.set_contract_address(COUNTER_CONTRACT_ADDRESS)?.build()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ContractInfo {
    pub block_timestamp: u64,
    pub contract_address: FieldElement,
    pub caller_address: FieldElement,
}

#[async_trait]
trait Counter {
    async fn incr(&self) -> Result<TxHash>;
    async fn dec(&self) -> Result<TxHash>;
    async fn register_address(&self, user_address: FieldElement) -> Result<TxHash>;
    async fn get_counter(&self) -> Result<u64>;
    async fn is_registered(&self, user_address: FieldElement) -> Result<bool>;
    async fn get_counter_status(&self) -> Result<ContractInfo>;
}

#[async_trait]
impl<T: Callable + Sync> Counter for T {
    async fn incr(&self) -> Result<TxHash> {
        self.invoke("incr", ()).await
    }

    async fn dec(&self) -> Result<TxHash> {
        self.invoke("dec", ()).await
    }

    async fn register_address(&self, user_address: FieldElement) -> Result<TxHash> {
        self.invoke("register_address", user_address).await
    }

    async fn get_counter(&self) -> Result<u64> {
        let ret = self.call("get_counter", ()).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn is_registered(&self, user_address: FieldElement) -> Result<bool> {
        let ret = self.call("is_registered", user_address).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn get_counter_status(&self) -> Result<ContractInfo> {
        let ret = self.call("get_counter_status", ()).await?;
        Ok(from_slice(ret.as_slice())?)
    }
}

#[tokio::test]
async fn test_counter_contract() {
    let counter = contract().unwrap();
    let _value = counter.get_counter().await.unwrap();
    // sleep(Duration::from_secs(1));
    assert!(counter.incr().await.is_ok());
    let address = FieldElement::from_str(ACCOUNT).unwrap();
    let is = counter.is_registered(address).await.unwrap();
    assert!(is)
}
