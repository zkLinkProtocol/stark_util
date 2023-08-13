use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use async_trait::async_trait;
use starknet::{
    core::{chain_id, types::FieldElement},
    providers::Provider,
};

use crate::provider::ExtendedProvider;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Network {
    Mainnet,
    #[default]
    Goerli1,
    Goerli2,
    Integration,
}

impl From<Network> for FieldElement {
    fn from(net: Network) -> FieldElement {
        match net {
            Network::Mainnet => chain_id::MAINNET,
            Network::Goerli1 => chain_id::TESTNET,
            Network::Goerli2 => chain_id::TESTNET2,
            Network::Integration => chain_id::TESTNET,
        }
    }
}

impl From<FieldElement> for Network {
    fn from(id: FieldElement) -> Network {
        if id == chain_id::MAINNET {
            Network::Mainnet
        } else if id == chain_id::TESTNET {
            Network::Goerli1
        } else if id == chain_id::TESTNET2 {
            Network::Goerli2
        } else {
            Network::Integration
        }
    }
}

#[async_trait]
pub trait NetworkSource {
    async fn get_network(&self) -> Result<Network>;
}

impl FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "mainnet" | "alpha-mainnet" => Ok(Self::Mainnet),
            "goerli" | "goerli1" | "goerli-1" | "alpha-goerli" | "alpha-goerli1" | "alpha-goerli-1" => Ok(Self::Goerli1),
            "goerli2" | "goerli-2" | "alpha-goerli2" | "alpha-goerli-2" => Ok(Self::Goerli2),
            "integration" => Ok(Self::Integration),
            _ => Err(anyhow::anyhow!("unknown network: {}", s)),
        }
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mainnet => write!(f, "mainnet"),
            Self::Goerli1 => write!(f, "goerli-1"),
            Self::Goerli2 => write!(f, "goerli-2"),
            Self::Integration => write!(f, "integration"),
        }
    }
}

#[async_trait]
impl NetworkSource for ExtendedProvider {
    async fn get_network(&self) -> Result<Network> {
        let chain_id = self.chain_id().await?;
        let is_integration = self.is_integration();
        Ok(if is_integration && chain_id == chain_id::TESTNET { Network::Integration } else { Network::from(chain_id) })
    }
}
