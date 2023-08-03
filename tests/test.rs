// privatekey 0x029d821d79d49716c0760c79a3258f25c84875476cd7db2afce1856162715976
// PUBLIC_KEY 0x2abb6a1b2cd549fdf87835d1c04a8c156ac4d42441b5b7fcb1386768558a7be
//
// ACCOUNT 0x5686c52b6f38639eb9cfb3dfff1b3260315099aa045fcc0b4a865068ba36aad
// {
//   "version": 1,
//   "variant": {
//     "type": "open_zeppelin",
//     "version": 1,
//     "PUBLIC_KEY": "0x2abb6a1b2cd549fdf87835d1c04a8c156ac4d42441b5b7fcb1386768558a7be"
//   },
//   "deployment": {
//     "status": "deployed",
//     "class_hash": "0x48dd59fabc729a5db3afdf649ecaf388e931647ab2f53ca3c6183fa480aa292",
//     "address": "0x5686c52b6f38639eb9cfb3dfff1b3260315099aa045fcc0b4a865068ba36aad"
//   }
// }
//
// ACCOUNT1 0x0131159c04f780f71bd16c0f453f25f769d80878b8ddeffcccbe24211b9bbd62
// {
//   "version": 1,
//   "variant": {
//     "type": "open_zeppelin",
//     "version": 1,
//     "PUBLIC_KEY": "0x2abb6a1b2cd549fdf87835d1c04a8c156ac4d42441b5b7fcb1386768558a7be"
//   },
//   "deployment": {
//     "status": "deployed",
//     "class_hash": "0x48dd59fabc729a5db3afdf649ecaf388e931647ab2f53ca3c6183fa480aa292",
//     "address": "0x131159c04f780f71bd16c0f453f25f769d80878b8ddeffcccbe24211b9bbd62"
//   }
// }
//
// ACCOUNT2 0x06ac7d3ef3458c6372e9f0dbb32c8bd023f0d5f7a98c650b105a5cc6dbde1cbc
// {
//   "version": 1,
//   "variant": {
//     "type": "open_zeppelin",
//     "version": 1,
//     "PUBLIC_KEY": "0x2abb6a1b2cd549fdf87835d1c04a8c156ac4d42441b5b7fcb1386768558a7be"
//   },
//   "deployment": {
//     "status": "deployed",
//     "class_hash": "0x48dd59fabc729a5db3afdf649ecaf388e931647ab2f53ca3c6183fa480aa292",
//     "address": "0x6ac7d3ef3458c6372e9f0dbb32c8bd023f0d5f7a98c650b105a5cc6dbde1cbc"
//   }
//
// ACCOUNT3 0x065f81fa8f222be104e463afc51bc97ea0d93d21e0bbfbfdca18c713c84a544f
// {
//   "version": 1,
//   "variant": {
//     "type": "open_zeppelin",
//     "version": 1,
//     "PUBLIC_KEY": "0x2abb6a1b2cd549fdf87835d1c04a8c156ac4d42441b5b7fcb1386768558a7be"
//   },
//   "deployment": {
//     "status": "deployed",
//     "class_hash": "0x48dd59fabc729a5db3afdf649ecaf388e931647ab2f53ca3c6183fa480aa292",
//     "address": "0x65f81fa8f222be104e463afc51bc97ea0d93d21e0bbfbfdca18c713c84a544f"
//   }
// }

// starkli deploy --ACCOUNT ~/.starkli-wallets/deployer/ACCOUNT.json --keystore  ~/.starkli-wallets/deployer/keystore.json --keystore-password 1314 0x059c3774e1f512cffb3f606943b44d1ce1fbe4c3de5daa95c5568f3aaaa27286 0x131159c04f780f71bd16c0f453f25f769d80878b8ddeffcccbe24211b9bbd62 0x65f81fa8f222be104e463afc51bc97ea0d93d21e0bbfbfdca18c713c84a544f 0x6ac7d3ef3458c6372e9f0dbb32c8bd023f0d5f7a98c650b105a5cc6dbde1cbc
// Deploying class 0x059c3774e1f512cffb3f606943b44d1ce1fbe4c3de5daa95c5568f3aaaa27286 with salt 0x00a183dcded805e255492e27a886120fa6ce16a99786669df82c066004b87aee...
// The contract will be deployed at address 0x03f99846b75acbe56129d5137403697774e681a35b9bf6f19cd59cb2fa62299c
// Contract deployment transaction: 0x029698f91055cfd901d9f2440d95c0830e08c6586ecbcf1c5f608be9884e73c2
// Contract deployed:
// 0x03f99846b75acbe56129d5137403697774e681a35b9bf6f19cd59cb2fa62299c

// starkli call 0x03f99846b75acbe56129d5137403697774e681a35b9bf6f19cd59cb2fa62299c  voter_can_vote 0x131159c04f780f71bd16c0f453f25f769d80878b8ddeffcccbe24211b9bbd62

use anyhow::Result;
use async_trait::async_trait;
use stark_util::{builder, client, client::StarkClient, from_slice, primitive::FieldElement};
use std::str::FromStr;

const PRIVATE_KEY: &str = "0x029d821d79d49716c0760c79a3258f25c84875476cd7db2afce1856162715976";
const PUBLIC_KEY: &str = "0x2abb6a1b2cd549fdf87835d1c04a8c156ac4d42441b5b7fcb1386768558a7be";
const ACCOUNT: &str = "0x5686c52b6f38639eb9cfb3dfff1b3260315099aa045fcc0b4a865068ba36aad";

const ACCOUNT1: &str = "0x0131159c04f780f71bd16c0f453f25f769d80878b8ddeffcccbe24211b9bbd62";
const ACCOUNT2: &str = "0x06ac7d3ef3458c6372e9f0dbb32c8bd023f0d5f7a98c650b105a5cc6dbde1cbc";
const ACCOUNT3: &str = "0x065f81fa8f222be104e463afc51bc97ea0d93d21e0bbfbfdca18c713c84a544f";
const VOTE_CONTRACT_ADDRESS: &str = "0x03f99846b75acbe56129d5137403697774e681a35b9bf6f19cd59cb2fa62299c";

fn stark_client() -> Result<client::StarkClient> {
    let builder = builder::Builder::new();
    builder.set_private_key(PRIVATE_KEY)?
           .set_owner_address(ACCOUNT)?
           .set_contract_address(VOTE_CONTRACT_ADDRESS)?
           .build()
}

#[async_trait]
trait Vote {
    async fn get_vote_status(&self) -> Result<(u8, u8, u8, u8)>;
    async fn voter_can_vote(&self, user_address: FieldElement) -> Result<bool>;
    async fn is_voter_registered(&self, user_address: FieldElement) -> Result<bool>;
    async fn vote(&self, vote: bool) -> Result<()>;
}

struct VoteContract {
    client: StarkClient,
}

impl From<StarkClient> for VoteContract {
    fn from(client: StarkClient) -> Self {
        VoteContract { client }
    }
}

#[async_trait]
impl Vote for VoteContract {
    async fn get_vote_status(&self) -> Result<(u8, u8, u8, u8)> {
        let ret = self.client.call("get_vote_status", ()).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn voter_can_vote(&self, user_address: FieldElement) -> Result<bool> {
        let ret = self.client.call("voter_can_vote", user_address).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn is_voter_registered(&self, user_address: FieldElement) -> Result<bool> {
        let ret = self.client.call("is_voter_registered", user_address).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn vote(&self, vote: bool) -> Result<()> {
        self.client.call("vote", vote).await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_vote() {
    let client = stark_client().unwrap();
    let vote: VoteContract = client.into();

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
