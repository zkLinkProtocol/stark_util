use url::Url;

use anyhow::Result;
use starknet::accounts::SingleOwnerAccount;
use starknet::core::types::FieldElement;
use starknet::signers::{LocalWallet, SigningKey};

use crate::client::StarkClient;
use crate::network::Network;
use crate::provider::ProviderArgs;

#[derive(Clone, Default, Debug)]
pub struct Builder {
    url: Option<Url>,
    is_rpc: bool,
    private_key: FieldElement,
    owner_address: FieldElement,
    // contract_address: &'static str,
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

    pub fn set_network(mut self, network: Network) -> Result<Self> {
        self.network = network;
        Ok(self)
    }

    pub fn build(self) -> Result<StarkClient> {
        let url = self.url.expect("url error");
        let provider = if self.is_rpc {
            ProviderArgs::Rpc(url).into()
        } else {
            let gateway_url = url.join("gateway")?;
            let feeder_gateway_url = url.join("feeder_gateway")?;
            ProviderArgs::Gateway(Some((gateway_url, feeder_gateway_url)), self.network).into()
        };

        let wallet = LocalWallet::from(SigningKey::from_secret_scalar(self.private_key));
        let owner =
            SingleOwnerAccount::new(provider, wallet, self.owner_address, self.network.into());

        Ok(StarkClient::new(
            owner,
            FieldElement::default(), //TODO
            self.owner_address,
        ))
    }
}
