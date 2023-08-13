#![allow(clippy::module_inception)]
use starknet::{
    core::{
        chain_id,
        types::{
            DeclareTransactionResult, DeployAccountTransactionResult, DeployTransactionResult, FieldElement,
            InvokeTransactionResult,
        },
    },
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, SequencerGatewayProvider},
};
use url::Url;

pub enum ProviderArgs {
    Rpc(Url),
    Gateway(Option<(Url, Url)>, FieldElement),
}

impl ProviderArgs {
    pub fn new(url: &str, network: FieldElement, is_rpc: bool) -> Self {
        let url = url.parse().expect("Url error");
        if is_rpc {
            ProviderArgs::Rpc(url)
        } else {
            let mut web_url = None;
            if let (Ok(gateway), Ok(feeder_gateway)) = (url.join("gateway"), url.join("feeder_gateway")) {
                web_url = Some((gateway, feeder_gateway));
            }
            ProviderArgs::Gateway(web_url, network)
        }
    }

    pub fn provider(self) -> AnyProvider {
        match self {
            ProviderArgs::Rpc(url) => AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(url))),
            ProviderArgs::Gateway(op, network) => match op {
                Some((gateway_url, feeder_gateway_url)) => {
                    AnyProvider::SequencerGateway(SequencerGatewayProvider::new(gateway_url,
                                                                                feeder_gateway_url,
                                                                                network))
                }
                None => {
                    let seq = if network == chain_id::MAINNET {
                        SequencerGatewayProvider::starknet_alpha_mainnet()
                    } else if network == chain_id::TESTNET {
                        SequencerGatewayProvider::starknet_alpha_goerli()
                    } else if network == chain_id::TESTNET2 {
                        SequencerGatewayProvider::starknet_alpha_goerli_2()
                    } else {
                        SequencerGatewayProvider::new(
                            Url::parse("https://external.integration.starknet.io/gateway").unwrap(),
                            Url::parse("https://external.integration.starknet.io/feeder_gateway")
                                .unwrap(),
                            chain_id::TESTNET,
                        )
                    };
                    AnyProvider::SequencerGateway(seq)
                }
            },
        }
    }
}

impl Default for ProviderArgs {
    fn default() -> Self {
        ProviderArgs::Gateway(None, chain_id::TESTNET)
    }
}

pub enum TransactionResult {
    Declare(DeclareTransactionResult),
    Deploy(DeployTransactionResult),
    Invoke(InvokeTransactionResult),
    DeployAccount(DeployAccountTransactionResult),
}
