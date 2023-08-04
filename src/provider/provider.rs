#![allow(clippy::module_inception)]

use anyhow::Result;
use async_trait::async_trait;
use starknet::{
    core::{chain_id, types::*},
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider, ProviderError, SequencerGatewayProvider},
};
use url::Url;

use super::Network;

pub enum ProviderArgs {
    Rpc(Url),
    Gateway(Option<(Url, Url)>, Network),
}

impl Default for ProviderArgs {
    fn default() -> Self {
        ProviderArgs::Gateway(None, Network::Goerli1)
    }
}

/// We need this because integration network has the same chain ID as
/// `goerli-1`. We would otherwise has no way of telling them apart. We could
/// generally just ignore this, but it would actually cause issues when deciding
/// what Sierra compiler version to use depending on network, so we still
/// need this.
pub struct ExtendedProvider {
    provider: AnyProvider,
    is_integration: bool,
}

impl ExtendedProvider {
    pub fn new(provider: AnyProvider, is_integration: bool) -> Self {
        Self { provider, is_integration }
    }
    pub fn is_rpc(&self) -> bool {
        matches!(self.provider, AnyProvider::JsonRpcHttp(_))
    }

    pub fn is_integration(&self) -> bool {
        self.is_integration
    }
}

impl From<ProviderArgs> for ExtendedProvider {
    fn from(args: ProviderArgs) -> Self {
        let mut is_integration = false;
        match args {
            ProviderArgs::Rpc(url) => ExtendedProvider::new(AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(url))), is_integration),
            ProviderArgs::Gateway(op, network) => {
                let gateway = match op {
                    Some((gateway_url, feeder_gateway_url)) => {
                        let chain_id = match network {
                            Network::Mainnet => chain_id::MAINNET,
                            Network::Goerli1 => chain_id::TESTNET,
                            Network::Goerli2 => chain_id::TESTNET2,
                            Network::Integration => {
                                is_integration = true;
                                chain_id::TESTNET
                            }
                        };
                        SequencerGatewayProvider::new(gateway_url, feeder_gateway_url, chain_id)
                    }
                    None => match network {
                        Network::Mainnet => SequencerGatewayProvider::starknet_alpha_mainnet(),
                        Network::Goerli1 => SequencerGatewayProvider::starknet_alpha_goerli(),
                        Network::Goerli2 => SequencerGatewayProvider::starknet_alpha_goerli_2(),
                        Network::Integration => {
                            is_integration = true;
                            SequencerGatewayProvider::new(Url::parse("https://external.integration.starknet.io/gateway").unwrap(),
                                                          Url::parse("https://external.integration.starknet.io/feeder_gateway").unwrap(),
                                                          chain_id::TESTNET)
                        }
                    },
                };
                ExtendedProvider::new(AnyProvider::SequencerGateway(gateway), is_integration)
            }
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl Provider for ExtendedProvider {
    type Error = <AnyProvider as Provider>::Error;

    async fn get_block_with_tx_hashes<B>(&self, block_id: B) -> Result<MaybePendingBlockWithTxHashes, ProviderError<Self::Error>>
        where B: AsRef<BlockId> + Send + Sync {
        <AnyProvider as Provider>::get_block_with_tx_hashes(&self.provider, block_id).await
    }

    async fn get_block_with_txs<B>(&self, block_id: B) -> Result<MaybePendingBlockWithTxs, ProviderError<Self::Error>>
        where B: AsRef<BlockId> + Send + Sync {
        <AnyProvider as Provider>::get_block_with_txs(&self.provider, block_id).await
    }

    async fn get_state_update<B>(&self, block_id: B) -> Result<MaybePendingStateUpdate, ProviderError<Self::Error>>
        where B: AsRef<BlockId> + Send + Sync {
        <AnyProvider as Provider>::get_state_update(&self.provider, block_id).await
    }

    async fn get_storage_at<A, K, B>(&self, contract_address: A, key: K, block_id: B) -> Result<FieldElement, ProviderError<Self::Error>>
        where A: AsRef<FieldElement> + Send + Sync,
              K: AsRef<FieldElement> + Send + Sync,
              B: AsRef<BlockId> + Send + Sync
    {
        <AnyProvider as Provider>::get_storage_at(&self.provider, contract_address, key, block_id).await
    }

    async fn get_transaction_by_hash<H>(&self, transaction_hash: H) -> Result<Transaction, ProviderError<Self::Error>>
        where H: AsRef<FieldElement> + Send + Sync {
        <AnyProvider as Provider>::get_transaction_by_hash(&self.provider, transaction_hash).await
    }

    async fn get_transaction_by_block_id_and_index<B>(&self, block_id: B, index: u64) -> Result<Transaction, ProviderError<Self::Error>>
        where B: AsRef<BlockId> + Send + Sync {
        <AnyProvider as Provider>::get_transaction_by_block_id_and_index(&self.provider, block_id, index).await
    }

    async fn get_transaction_receipt<H>(&self, transaction_hash: H) -> Result<MaybePendingTransactionReceipt, ProviderError<Self::Error>>
        where H: AsRef<FieldElement> + Send + Sync {
        <AnyProvider as Provider>::get_transaction_receipt(&self.provider, transaction_hash).await
    }

    async fn get_class<B, H>(&self, block_id: B, class_hash: H) -> Result<ContractClass, ProviderError<Self::Error>>
        where B: AsRef<BlockId> + Send + Sync,
              H: AsRef<FieldElement> + Send + Sync
    {
        <AnyProvider as Provider>::get_class(&self.provider, block_id, class_hash).await
    }

    async fn get_class_hash_at<B, A>(&self, block_id: B, contract_address: A) -> Result<FieldElement, ProviderError<Self::Error>>
        where B: AsRef<BlockId> + Send + Sync,
              A: AsRef<FieldElement> + Send + Sync
    {
        <AnyProvider as Provider>::get_class_hash_at(&self.provider, block_id, contract_address).await
    }

    async fn get_class_at<B, A>(&self, block_id: B, contract_address: A) -> Result<ContractClass, ProviderError<Self::Error>>
        where B: AsRef<BlockId> + Send + Sync,
              A: AsRef<FieldElement> + Send + Sync
    {
        <AnyProvider as Provider>::get_class_at(&self.provider, block_id, contract_address).await
    }

    async fn get_block_transaction_count<B>(&self, block_id: B) -> Result<u64, ProviderError<Self::Error>>
        where B: AsRef<BlockId> + Send + Sync {
        <AnyProvider as Provider>::get_block_transaction_count(&self.provider, block_id).await
    }

    async fn call<R, B>(&self, request: R, block_id: B) -> Result<Vec<FieldElement>, ProviderError<Self::Error>>
        where R: AsRef<FunctionCall> + Send + Sync,
              B: AsRef<BlockId> + Send + Sync
    {
        <AnyProvider as Provider>::call(&self.provider, request, block_id).await
    }

    async fn estimate_fee<R, B>(&self, request: R, block_id: B) -> Result<Vec<FeeEstimate>, ProviderError<Self::Error>>
        where R: AsRef<[BroadcastedTransaction]> + Send + Sync,
              B: AsRef<BlockId> + Send + Sync
    {
        <AnyProvider as Provider>::estimate_fee(&self.provider, request, block_id).await
    }

    async fn block_number(&self) -> Result<u64, ProviderError<Self::Error>> {
        <AnyProvider as Provider>::block_number(&self.provider).await
    }

    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, ProviderError<Self::Error>> {
        <AnyProvider as Provider>::block_hash_and_number(&self.provider).await
    }

    async fn chain_id(&self) -> Result<FieldElement, ProviderError<Self::Error>> {
        <AnyProvider as Provider>::chain_id(&self.provider).await
    }

    async fn pending_transactions(&self) -> Result<Vec<Transaction>, ProviderError<Self::Error>> {
        <AnyProvider as Provider>::pending_transactions(&self.provider).await
    }

    async fn syncing(&self) -> Result<SyncStatusType, ProviderError<Self::Error>> {
        <AnyProvider as Provider>::syncing(&self.provider).await
    }

    async fn get_events(&self,
                        filter: EventFilter,
                        continuation_token: Option<String>,
                        chunk_size: u64)
                        -> Result<EventsPage, ProviderError<Self::Error>> {
        <AnyProvider as Provider>::get_events(&self.provider, filter, continuation_token, chunk_size).await
    }

    async fn get_nonce<B, A>(&self, block_id: B, contract_address: A) -> Result<FieldElement, ProviderError<Self::Error>>
        where B: AsRef<BlockId> + Send + Sync,
              A: AsRef<FieldElement> + Send + Sync
    {
        <AnyProvider as Provider>::get_nonce(&self.provider, block_id, contract_address).await
    }

    async fn add_invoke_transaction<I>(&self, invoke_transaction: I) -> Result<InvokeTransactionResult, ProviderError<Self::Error>>
        where I: AsRef<BroadcastedInvokeTransaction> + Send + Sync {
        <AnyProvider as Provider>::add_invoke_transaction(&self.provider, invoke_transaction).await
    }

    async fn add_declare_transaction<D>(&self, declare_transaction: D) -> Result<DeclareTransactionResult, ProviderError<Self::Error>>
        where D: AsRef<BroadcastedDeclareTransaction> + Send + Sync {
        <AnyProvider as Provider>::add_declare_transaction(&self.provider, declare_transaction).await
    }

    async fn add_deploy_account_transaction<D>(&self,
                                               deploy_account_transaction: D)
                                               -> Result<DeployAccountTransactionResult, ProviderError<Self::Error>>
        where D: AsRef<BroadcastedDeployAccountTransaction> + Send + Sync
    {
        <AnyProvider as Provider>::add_deploy_account_transaction(&self.provider, deploy_account_transaction).await
    }
}
