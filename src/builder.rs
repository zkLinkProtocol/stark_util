use std::str::FromStr;

use anyhow::Result;
use starknet::{
    accounts::{ConnectedAccount, SingleOwnerAccount},
    core::types::FieldElement,
    signers::{LocalWallet, SigningKey},
};

use crate::{provider::provider::ProviderArgs, ContractInstance};

/// build contract
#[derive(Clone, Default, Debug)]
pub struct Builder {
    url: &'static str,
    is_rpc: bool,
    private_key: FieldElement,
    owner_address: FieldElement,
    contract_address: FieldElement,
    network: FieldElement,
}

impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }

    pub fn set_url(mut self, url: &'static str, is_rpc: bool) -> Result<Self> {
        self.url = url;
        self.is_rpc = is_rpc;
        Ok(self)
    }

    pub fn set_private_key(mut self, private_key: &str) -> Result<Self> {
        self.private_key = FieldElement::from_hex_be(private_key)?;
        Ok(self)
    }

    pub fn set_owner_address(mut self, address: &str) -> Result<Self> {
        self.owner_address = FieldElement::from_hex_be(address)?;
        Ok(self)
    }

    pub fn set_contract_address(mut self, address: &str) -> Result<Self> {
        self.contract_address = FieldElement::from_str(address)?;
        Ok(self)
    }

    pub fn set_network(mut self, network: FieldElement) -> Result<Self> {
        self.network = network;
        Ok(self)
    }

    pub fn build(self) -> Result<ContractInstance<impl ConnectedAccount>> {
        let provider = ProviderArgs::new(self.url, self.network, self.is_rpc).provider();
        let wallet = LocalWallet::from(SigningKey::from_secret_scalar(self.private_key));
        let owner = SingleOwnerAccount::new(provider, wallet, self.owner_address, self.network);

        Ok(ContractInstance::new(owner, self.contract_address))
    }
}
