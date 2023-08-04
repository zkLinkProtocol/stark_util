use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use stark_util::{
    builder, client,
    contract::{Callable, Contract},
    from_slice,
    primitive::FieldElement,
    proto::TxHash,
};

const PRIVATE_KEY: &str = "0x029d821d79d49716c0760c79a3258f25c84875476cd7db2afce1856162715976";
#[allow(dead_code)]
const PUBLIC_KEY: &str = "0x2abb6a1b2cd549fdf87835d1c04a8c156ac4d42441b5b7fcb1386768558a7be";
const ACCOUNT: &str = "0x5686c52b6f38639eb9cfb3dfff1b3260315099aa045fcc0b4a865068ba36aad";

const ACCOUNT1: &str = "0x0131159c04f780f71bd16c0f453f25f769d80878b8ddeffcccbe24211b9bbd62";
const ACCOUNT2: &str = "0x06ac7d3ef3458c6372e9f0dbb32c8bd023f0d5f7a98c650b105a5cc6dbde1cbc";
const ACCOUNT3: &str = "0x065f81fa8f222be104e463afc51bc97ea0d93d21e0bbfbfdca18c713c84a544f";
const VOTE_CONTRACT_ADDRESS: &str = "0x03f99846b75acbe56129d5137403697774e681a35b9bf6f19cd59cb2fa62299c";
const COUNTER_CONTRACT_ADDRESS: &str = "0x0311bb7385271f9fa3754218f4bf097a784c308da898df405b84d571f5ed7468";

fn stark_client() -> Result<client::StarkClient> {
    let builder = builder::Builder::new();
    builder.set_private_key(PRIVATE_KEY)?.set_owner_address(ACCOUNT)?.set_contract_address(VOTE_CONTRACT_ADDRESS)?.build()
}

#[async_trait]
trait Vote {
    async fn get_vote_status(&self) -> Result<(u8, u8, u8, u8)>;
    async fn voter_can_vote(&self, user_address: FieldElement) -> Result<bool>;
    async fn is_voter_registered(&self, user_address: FieldElement) -> Result<bool>;
    async fn vote(&self, vote: bool) -> Result<TxHash>;
}

#[async_trait]
impl<T: Callable + Sync> Vote for T {
    async fn get_vote_status(&self) -> Result<(u8, u8, u8, u8)> {
        let ret = self.call("get_vote_status", ()).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn voter_can_vote(&self, user_address: FieldElement) -> Result<bool> {
        let ret = self.call("voter_can_vote", user_address).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn is_voter_registered(&self, user_address: FieldElement) -> Result<bool> {
        let ret = self.call("is_voter_registered", user_address).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn vote(&self, vote: bool) -> Result<TxHash> {
        self.invoke("vote", vote).await
    }
}

#[tokio::test]
async fn test_vote_contract() {
    let client = stark_client().unwrap();
    let address = FieldElement::from_str(VOTE_CONTRACT_ADDRESS).unwrap();
    let vote = Contract::new(client, address);

    let is = vote.is_voter_registered(FieldElement::from_str(ACCOUNT).unwrap()).await.unwrap();
    assert_eq!(is, false);
    let is = vote.is_voter_registered(FieldElement::from_str(ACCOUNT1).unwrap()).await.unwrap();
    assert!(is);
    let is = vote.is_voter_registered(FieldElement::from_str(ACCOUNT2).unwrap()).await.unwrap();
    assert!(is);
    let is = vote.is_voter_registered(FieldElement::from_str(ACCOUNT3).unwrap()).await.unwrap();
    assert!(is);

    let is = vote.voter_can_vote(FieldElement::from_str(ACCOUNT).unwrap()).await.unwrap();
    assert_eq!(is, false);
    let is = vote.voter_can_vote(FieldElement::from_str(ACCOUNT2).unwrap()).await.unwrap();
    assert!(is);
    let is = vote.voter_can_vote(FieldElement::from_str(ACCOUNT3).unwrap()).await.unwrap();
    assert!(is);

    let (yes, no, yes_percentage, no_percentage) = vote.get_vote_status().await.unwrap();
    assert_eq!(yes, 1);
    assert_eq!(no, 0);
    assert_eq!(yes_percentage, 100);
    assert_eq!(no_percentage, 0);
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
    let client = stark_client().unwrap();
    let contract_address = FieldElement::from_str(COUNTER_CONTRACT_ADDRESS).unwrap();
    let counter = Contract::new(client, contract_address);
    let _value = counter.get_counter().await.unwrap();
    // sleep(Duration::from_secs(1));
    assert!(counter.incr().await.is_ok());
    let address = FieldElement::from_str(ACCOUNT).unwrap();
    let is = counter.is_registered(address).await.unwrap();
    assert!(is)
}

