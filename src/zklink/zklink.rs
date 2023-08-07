#![allow(clippy::module_inception)]
use anyhow::Result;
use async_trait::async_trait;
use starknet_api::core::ContractAddress;

use crate::{contract::Callable, from_slice, primitive::*};
use super::model::*;

#[async_trait]
pub trait ZkLink {
    // =================User interface=================
    async fn deposit_erc20(&mut self,
                           token: ContractAddress,
                           amount: u128,
                           zk_link_address: ContractAddress,
                           sub_account_id: u8,
                           mapping: bool)
                           -> Result<TxHash>;

    async fn transfer_erc20(&mut self,
                            token: ContractAddress,
                            to: ContractAddress,
                            amount: u128,
                            max_amount: u128,
                            is_standard: bool)
                            -> Result<TxHash>;

    async fn request_full_exit(&mut self, account_id: u32, sub_account_id: u8, token_id: u16, mapping: bool) -> Result<TxHash>;

    async fn activate_exodus_mode(&mut self) -> Result<TxHash>;

    #[allow(clippy::too_many_arguments)]
    async fn perform_exodus(&mut self,
                            stored_block_info: StoredBlockInfo,
                            owner: ContractAddress,
                            account_id: u32,
                            sub_account_id: u8,
                            withdraw_token_id: u16,
                            deduct_token_id: u16,
                            amount: u128,
                            proof: Vec<U256>)
                            -> Result<TxHash>;

    async fn cancel_out_standing_deposits_for_exodus_ode(&mut self, n: u64, deposits_pubdata: Vec<Bytes>) -> Result<TxHash>;

    async fn set_auth_pubkey_hash(&mut self, pubkey_hash: Felt252, nonce: u32) -> Result<TxHash>;

    async fn withdraw_pending_balance(&mut self, owner: ContractAddress, token_id: u16, amount: u128) -> Result<TxHash>;

    async fn get_pending_balance(&self, address: ContractAddress, token_id: u16) -> Result<u128>;

    // =================Validator interface=================
    async fn commit_blocks(&mut self, last_committed_block_data: StoredBlockInfo, new_blocks_data: Vec<CommitBlockInfo>) -> Result<TxHash>;

    /// Blocks commitment verification.
    /// Only verifies block commitments without any other processing
    async fn commit_compressed_blocks(&mut self,
                                      last_committed_block_data: StoredBlockInfo,
                                      new_blocks_data: Vec<CommitBlockInfo>,
                                      new_blocks_extra_data: Vec<CompressedBlockExtraInfo>)
                                      -> Result<TxHash>;

    async fn execute_blocks(&mut self, blocks_data: Vec<ExecuteBlockInfo>) -> Result<TxHash>;

    // =================Block interface=====================
    async fn prove_blocks(&mut self, committed_blocks: Vec<StoredBlockInfo>, proof: ProofInput) -> Result<TxHash>;

    async fn revert_blocks(&mut self, blocks_to_revert: Vec<StoredBlockInfo>) -> Result<TxHash>;

    // =================Cross chain block synchronization===============
    async fn receive_synchronization_progress(&mut self, sync_hash: U256, progress: U256) -> Result<TxHash>;

    async fn get_synchronized_progress(&self, block: StoredBlockInfo) -> Result<U256>;

    async fn sync_blocks(&mut self, block: StoredBlockInfo) -> Result<TxHash>;

    // =================Fast withdraw and Accept===============
    #[allow(clippy::too_many_arguments)]
    async fn accept_erc20(&mut self,
                          accepter: ContractAddress,
                          account_id: u32,
                          receiver: ContractAddress,
                          token_id: u16,
                          amount: u128,
                          withdraw_fee_rate: u16,
                          nonce: u32,
                          amount_transfer: u128)
                          -> Result<TxHash>;

    async fn broker_allowance(&self, token_id: u16, accepter: ContractAddress, broker: ContractAddress) -> Result<u128>;

    async fn broker_approve(&mut self, token_id: u16, broker: ContractAddress, amount: u128) -> Result<TxHash>;

    // =================Governance interface===============
    async fn change_governor(&mut self, new_governor: ContractAddress) -> Result<TxHash>;

    async fn add_token(&mut self, token_id: u16, token_address: ContractAddress, decimals: u8, standard: bool) -> Result<TxHash>;

    async fn add_tokens(&mut self, token_list: Vec<Token>) -> Result<TxHash>;

    async fn set_token_paused(&mut self, token_id: u16, token_paused: bool) -> Result<TxHash>;

    async fn set_validator(&mut self, validator: ContractAddress, active: bool) -> Result<TxHash>;

    async fn add_bridge(&mut self, bridge: ContractAddress) -> Result<TxHash>;

    async fn update_bridge(&mut self, index: usize, enable_bridge_to: bool, enable_bridge_from: bool) -> Result<TxHash>;

    async fn is_bridge_to_enabled(&self, bridge: ContractAddress) -> Result<bool>;

    async fn is_bridge_from_enabled(&self, bridge: ContractAddress) -> Result<bool>;

    async fn network_governor(&self) -> Result<ContractAddress>;
}

#[async_trait]
impl<T: Callable + Sync + Send> ZkLink for T {
    async fn deposit_erc20(&mut self,
                           token: ContractAddress,
                           amount: u128,
                           zk_link_address: ContractAddress,
                           sub_account_id: u8,
                           mapping: bool)
                           -> Result<TxHash> {
        self.invoke("depositERC20", (token, amount, zk_link_address, sub_account_id, mapping)).await
    }

    async fn transfer_erc20(&mut self,
                            token: ContractAddress,
                            to: ContractAddress,
                            amount: u128,
                            max_amount: u128,
                            is_standard: bool)
                            -> Result<TxHash> {
        self.invoke("transferERC20", (token, to, amount, max_amount, is_standard)).await
    }

    async fn request_full_exit(&mut self, account_id: u32, sub_account_id: u8, token_id: u16, mapping: bool) -> Result<TxHash> {
        self.invoke("requestFullExit", (account_id, sub_account_id, token_id, mapping)).await
    }

    async fn activate_exodus_mode(&mut self) -> Result<TxHash> {
        self.invoke("activateExodusMode", ()).await
    }

    #[rustfmt::skip]
    async fn perform_exodus(&mut self,
                            stored_block_info: StoredBlockInfo,
                            owner: ContractAddress,
                            account_id: u32,
                            sub_account_id: u8,
                            withdraw_token_id: u16,
                            deduct_token_id: u16,
                            amount: u128,
                            proof: Vec<U256>)
                            -> Result<TxHash> {
        self.invoke("performExodus", (stored_block_info, owner, account_id, sub_account_id, withdraw_token_id, deduct_token_id, amount, proof)).await
    }

    async fn cancel_out_standing_deposits_for_exodus_ode(&mut self, n: u64, deposits_pubdata: Vec<Bytes>) -> Result<TxHash> {
        self.invoke("cancelOutstandingDepositsForExodusMode", (n, deposits_pubdata)).await
    }

    async fn set_auth_pubkey_hash(&mut self, pubkey_hash: Felt252, nonce: u32) -> Result<TxHash> {
        self.invoke("setAuthPubkeyHash", (pubkey_hash, nonce)).await
    }

    async fn withdraw_pending_balance(&mut self, owner: ContractAddress, token_id: u16, amount: u128) -> Result<TxHash> {
        self.invoke("withdrawPendingBalance", (owner, token_id, amount)).await
    }

    async fn get_pending_balance(&self, address: ContractAddress, token_id: u16) -> Result<u128> {
        let ret = self.call("getPendingBalance", (address, token_id)).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    // =================Validator interface=================
    async fn commit_blocks(&mut self, last_committed_block_data: StoredBlockInfo, new_blocks_data: Vec<CommitBlockInfo>) -> Result<TxHash> {
        self.invoke("commitBlocks", (last_committed_block_data, new_blocks_data)).await
    }
    /// Blocks commitment verification.
    /// Only verifies block commitments without any other processing
    async fn commit_compressed_blocks(&mut self,
                                      last_committed_block_data: StoredBlockInfo,
                                      new_blocks_data: Vec<CommitBlockInfo>,
                                      new_blocks_extra_data: Vec<CompressedBlockExtraInfo>)
                                      -> Result<TxHash> {
        self.invoke("commitCompressedBlocks", (last_committed_block_data, new_blocks_data, new_blocks_extra_data)).await
    }

    async fn execute_blocks(&mut self, blocks_data: Vec<ExecuteBlockInfo>) -> Result<TxHash> {
        self.invoke("executeBlocks", blocks_data).await
    }

    // =================Block interface=====================
    async fn prove_blocks(&mut self, committed_blocks: Vec<StoredBlockInfo>, proof: ProofInput) -> Result<TxHash> {
        self.invoke("proveBlocks", (committed_blocks, proof)).await
    }

    async fn revert_blocks(&mut self, blocks_to_revert: Vec<StoredBlockInfo>) -> Result<TxHash> {
        self.invoke("revertBlocks", blocks_to_revert).await
    }

    // =================Cross chain block synchronization===============
    async fn receive_synchronization_progress(&mut self, sync_hash: U256, progress: U256) -> Result<TxHash> {
        self.invoke("receiveSynchronizationProgress", (sync_hash, progress)).await
    }

    async fn get_synchronized_progress(&self, block: StoredBlockInfo) -> Result<U256> {
        let ret = self.call("getSynchronizedProgress", block).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn sync_blocks(&mut self, block: StoredBlockInfo) -> Result<TxHash> {
        self.invoke("syncBlocks", block).await
    }

    // =================Fast withdraw and Accept===============
    async fn accept_erc20(&mut self,
                          accepter: ContractAddress,
                          account_id: u32,
                          receiver: ContractAddress,
                          token_id: u16,
                          amount: u128,
                          withdraw_fee_rate: u16,
                          nonce: u32,
                          amount_transfer: u128)
                          -> Result<TxHash> {
        self.invoke("acceptERC20", (accepter, account_id, receiver, token_id, amount, withdraw_fee_rate, nonce, amount_transfer)).await
    }

    async fn broker_allowance(&self, token_id: u16, accepter: ContractAddress, broker: ContractAddress) -> Result<u128> {
        let ret = self.call("brokerAllowance", (token_id, accepter, broker)).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn broker_approve(&mut self, token_id: u16, broker: ContractAddress, amount: u128) -> Result<TxHash> {
        self.invoke("brokerApprove", (token_id, broker, amount)).await
    }

    // =================Governance interface===============
    async fn change_governor(&mut self, new_governor: ContractAddress) -> Result<TxHash> {
        self.invoke("changeGovernor", new_governor).await
    }

    async fn add_token(&mut self, token_id: u16, token_address: ContractAddress, decimals: u8, standard: bool) -> Result<TxHash> {
        self.invoke("addToken", (token_id, token_address, decimals, standard)).await
    }

    async fn add_tokens(&mut self, token_list: Vec<Token>) -> Result<TxHash> {
        self.invoke("addTokens", token_list).await
    }

    async fn set_token_paused(&mut self, token_id: u16, token_paused: bool) -> Result<TxHash> {
        self.invoke("setTokenPaused", (token_id, token_paused)).await
    }

    async fn set_validator(&mut self, validator: ContractAddress, active: bool) -> Result<TxHash> {
        self.invoke("setValidator", (validator, active)).await
    }

    async fn add_bridge(&mut self, bridge: ContractAddress) -> Result<TxHash> {
        self.invoke("addBridge", bridge).await
    }

    async fn update_bridge(&mut self, index: usize, enable_bridge_to: bool, enable_bridge_from: bool) -> Result<TxHash> {
        self.invoke("updateBridge", (index, enable_bridge_to, enable_bridge_from)).await
    }

    async fn is_bridge_to_enabled(&self, bridge: ContractAddress) -> Result<bool> {
        let ret = self.call("isBridgeToEnabled", bridge).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn is_bridge_from_enabled(&self, bridge: ContractAddress) -> Result<bool> {
        let ret = self.call("isBridgeFromEnabled", bridge).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn network_governor(&self) -> Result<ContractAddress> {
        let ret = self.call("networkGovernor", ()).await?;
        Ok(from_slice(ret.as_slice())?)
    }
}
