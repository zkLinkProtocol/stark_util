#![allow(dead_code)]

use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
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
const VOTE_CONTRACT_ADDRESS: &str = "0x03f99846b75acbe56129d5137403697774e681a35b9bf6f19cd59cb2fa62299c";

fn contract() -> Result<Contract> {
    let builder = Builder::new();
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
    let vote = contract().unwrap();

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
