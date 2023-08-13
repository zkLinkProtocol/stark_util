use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use starknet::{accounts::ConnectedAccount, providers::Provider};

use crate::{
    primitive::{FieldElement, TxHash},
    provider::{caller::Caller, invoker::Invoker},
};

pub struct ContractInstance<Account> {
    account: Account,
    address: FieldElement,
}

impl<Account> ContractInstance<Account> {
    pub fn new(account: Account, address: FieldElement) -> Self {
        ContractInstance { account, address }
    }

    pub fn set_address(&mut self, address: FieldElement) {
        self.address = address
    }
}

#[async_trait]
pub trait ContractCaller {
    type Provider: Provider + Sync;

    fn provider(&self) -> &Self::Provider;

    fn address(&self) -> FieldElement;

    async fn call<I, O>(&self, func_name: &str, calldata: I) -> anyhow::Result<O>
        where I: Serialize + Send,
              O: DeserializeOwned + Send,
              <<Self as ContractCaller>::Provider as Provider>::Error: 'static
    {
        self.provider().caller(self.address(), func_name, calldata).await
    }
}

#[async_trait]
impl<Account: ConnectedAccount> ContractCaller for ContractInstance<Account> {
    type Provider = Account::Provider;

    fn provider(&self) -> &Self::Provider {
        self.account.provider()
    }

    fn address(&self) -> FieldElement {
        self.address
    }
}

#[async_trait]
pub trait ContractInvoker {
    type Account: ConnectedAccount + Send + Sync + 'static;

    fn account(&self) -> &Self::Account;

    fn address(&self) -> FieldElement;

    async fn invoke<T>(&self, func_name: &str, calldata: T) -> anyhow::Result<TxHash>
        where T: Serialize + Send
    {
        self.account().invoker(self.address(), func_name, calldata).await
    }
}

#[async_trait]
impl<A: ConnectedAccount + Send + Sync + 'static> ContractInvoker for ContractInstance<A> {
    type Account = A;
    fn account(&self) -> &Self::Account {
        &self.account
    }

    fn address(&self) -> FieldElement {
        self.address
    }
}
