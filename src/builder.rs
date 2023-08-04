use std::str::FromStr;

use url::Url;
use anyhow::Result;
use starknet::{
    accounts::SingleOwnerAccount,
    core::types::FieldElement,
    signers::{LocalWallet, SigningKey},
};

use crate::{client::StarkClient, network::Network, provider::ProviderArgs};

/// build contract
#[derive(Clone, Default, Debug)]
pub struct Builder {
    url: Option<Url>,
    is_rpc: bool,
    private_key: FieldElement,
    owner_address: FieldElement,
    contract_address: FieldElement,
    network: Network,
}

impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }

    pub fn set_url(mut self, url: &str, is_rpc: bool) -> Result<Self> {
        let url = url.parse()?;
        self.url = Some(url);
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

    pub fn set_network(mut self, network: Network) -> Result<Self> {
        self.network = network;
        Ok(self)
    }

    pub fn build(self) -> Result<StarkClient> {
        let provider = if self.is_rpc {
            let url = self.url.expect("url error");
            ProviderArgs::Rpc(url).into()
        } else {
            let web_url = match self.url {
                Some(url) => Some((url.join("gateway")?, url.join("feeder_gateway")?)),
                _ => None,
            };
            ProviderArgs::Gateway(web_url, self.network).into()
        };

        let wallet = LocalWallet::from(SigningKey::from_secret_scalar(self.private_key));
        let owner = SingleOwnerAccount::new(provider, wallet, self.owner_address, self.network.into());

        Ok(StarkClient::new(owner, self.owner_address))
    }
}
