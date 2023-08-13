use anyhow::Result;
#[allow(unused_imports)]
use stark_util::{
    zklink::{ZkLink, ZkLinkTest},
    Builder, Contract,
};

const PRIVATE_KEY: &str = "0x029d821d79d49716c0760c79a3258f25c84875476cd7db2afce1856162715976";
const ACCOUNT: &str = "0x5686c52b6f38639eb9cfb3dfff1b3260315099aa045fcc0b4a865068ba36aad";
// contract address
const COUNTER_CONTRACT_ADDRESS: &str = "0x0311bb7385271f9fa3754218f4bf097a784c308da898df405b84d571f5ed7468";

fn contract() -> Result<Contract> {
    let builder = Builder::new();
    builder.set_private_key(PRIVATE_KEY)?.set_owner_address(ACCOUNT)?.set_contract_address(COUNTER_CONTRACT_ADDRESS)?.build()
}

#[tokio::main]
async fn main() {
    println!("call contract");
    let zk = contract().unwrap();
    let ret = zk.u8s_test1(vec![1]).await.unwrap();
    println!("u8s_test1 result: {}", ret);

    let ret = zk.u8s_test2(vec![1]).await.unwrap();
    println!("u8s_test2 result: {:?}", ret);
}
