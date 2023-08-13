use serde::Serialize;
use starknet::{
    accounts::{Account, Call, ConnectedAccount, SingleOwnerAccount},
    core::{
        types::{BlockId, BlockTag, FieldElement, FunctionCall},
        utils::get_selector_from_name,
    },
    providers::Provider,
    signers::LocalWallet,
};

use crate::{primitive::TxHash, provider::ExtendedProvider, to_field_elements};

pub struct StarkClient {
    owner: SingleOwnerAccount<ExtendedProvider, LocalWallet>,
}

impl StarkClient {
    pub fn new(owner: SingleOwnerAccount<ExtendedProvider, LocalWallet>) -> Self {
        Self { owner }
    }

    pub fn owner(&self) -> &SingleOwnerAccount<ExtendedProvider, LocalWallet> {
        &self.owner
    }

    pub async fn get_last_block_number(&self) -> anyhow::Result<u64> {
        Ok(self.owner().provider().block_number().await?)
    }

    pub async fn get_pending_nonce(&self, address: FieldElement) -> anyhow::Result<FieldElement> {
        Ok(self.owner().provider().get_nonce(BlockId::Tag(BlockTag::Pending), address).await?)
    }

    pub async fn invoke<T>(&self, contract_address: FieldElement, func_name: &str, calldata: T) -> anyhow::Result<TxHash>
        where T: Serialize {
        let selector = get_selector_from_name(func_name).unwrap();
        let calldata_elements = to_field_elements(calldata)?;
        let result = self.owner().execute(vec![Call { to: contract_address, selector, calldata: calldata_elements }]).send().await?;
        let tx_hash = result.transaction_hash;
        Ok(tx_hash.into())
    }

    pub async fn call<T>(&self, contract_address: FieldElement, func_name: &str, calldata: T) -> anyhow::Result<Vec<FieldElement>>
        where T: Serialize {
        let selector = get_selector_from_name(func_name)?;
        let calldata = to_field_elements(calldata)?;
        Ok(self.owner()
               .provider()
               .call(FunctionCall { contract_address, entry_point_selector: selector, calldata }, BlockId::Tag(BlockTag::Pending))
               .await?)
    }
}
