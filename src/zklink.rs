use crate::{contract::Callable, from_slice, primitive::*, proto::*, U256};
use anyhow::Result;
use async_trait::async_trait;
use starknet_api::core::ContractAddress;

#[async_trait]
pub trait ZkLink {
    // =================User interface=================
    async fn deposit_erc20(&mut self,
                           _token: ContractAddress,
                           _amount: u128,
                           _zk_link_address: ContractAddress,
                           _sub_account_id: u8,
                           _mapping: bool)
                           -> Result<TxHash>;

    async fn transfer_erc20(&mut self,
                            _token: ContractAddress,
                            _to: ContractAddress,
                            _amount: u128,
                            _max_amount: u128,
                            _is_standard: bool)
                            -> Result<TxHash>;

    async fn request_full_exit(&mut self, _account_id: u32, _sub_account_id: u8, _token_id: u16, _mapping: bool) -> Result<TxHash>;

    async fn activate_exodus_mode(&mut self) -> Result<TxHash>;

    async fn perform_exodus(&mut self,
                            _stored_block_info: StoredBlockInfo,
                            _owner: ContractAddress,
                            _account_id: u32,
                            _sub_account_id: u8,
                            _withdraw_token_id: u16,
                            _deduct_token_id: u16,
                            _amount: u128,
                            _proof: Vec<U256>)
                            -> Result<TxHash>;

    async fn set_auth_pubkey_hash(&mut self, _pubkey_hash: Felt252, _nonce: u32) -> Result<TxHash>;

    async fn withdraw_pending_balance(&mut self, _owner: ContractAddress, _token_id: u16, _amount: u128) -> Result<TxHash>;

    async fn get_pending_balance(&self, _address: ContractAddress, _token_id: u16) -> Result<u128>;

    // =================Validator interface=================
    async fn commit_blocks(&mut self, _last_committed_block_data: StoredBlockInfo, _new_blocks_data: Vec<CommitBlockInfo>) -> Result<TxHash>;

    /// Blocks commitment verification.
    /// Only verifies block commitments without any other processing
    async fn commit_compressed_blocks(&mut self,
                                      _last_committed_block_data: StoredBlockInfo,
                                      _new_blocks_data: Vec<CommitBlockInfo>,
                                      _new_blocks_extra_data: Vec<CompressedBlockExtraInfo>)
                                      -> Result<TxHash>;

    async fn execute_blocks(&mut self, _blocks_data: Vec<ExecuteBlockInfo>) -> Result<TxHash>;

    // =================Block interface=====================
    async fn prove_blocks(&mut self, _committed_blocks: Vec<StoredBlockInfo>, _proof: ProofInput) -> Result<TxHash>;

    async fn revert_blocks(&mut self, _blocks_to_revert: Vec<StoredBlockInfo>) -> Result<TxHash>;

    // =================Cross chain block synchronization===============
    async fn receive_synchronization_progress(&mut self, _sync_hash: U256, _progress: U256) -> Result<TxHash>;

    async fn get_synchronized_progress(&self, _block: StoredBlockInfo) -> Result<U256>;

    async fn sync_blocks(&mut self, _block: StoredBlockInfo) -> Result<TxHash>;

    // =================Fast withdraw and Accept===============
    async fn accept_erc20(&mut self,
                          _accepter: ContractAddress,
                          _account_id: u32,
                          _receiver: ContractAddress,
                          _token_id: u16,
                          _amount: u128,
                          _withdraw_fee_rate: u16,
                          _nonce: u32,
                          _amount_transfer: u128)
                          -> Result<TxHash>;

    async fn broker_allowance(&self, _token_id: u16, _accepter: ContractAddress, _broker: ContractAddress) -> Result<u128>;

    async fn broker_approve(&mut self, _token_id: u16, _broker: ContractAddress, _amount: u128) -> Result<TxHash>;

    // =================Governance interface===============
    async fn change_governor(&mut self, _new_governor: ContractAddress) -> Result<TxHash>;

    async fn add_token(&mut self, _token_id: u16, _token_address: ContractAddress, _decimals: u8, _standard: bool) -> Result<TxHash>;

    async fn add_tokens(&mut self, _token_list: Vec<Token>) -> Result<TxHash>;

    async fn set_token_paused(&mut self, _token_id: u16, _token_paused: bool) -> Result<TxHash>;

    async fn set_validator(&mut self, _validator: ContractAddress, _active: bool) -> Result<TxHash>;

    async fn add_bridge(&mut self, _bridge: ContractAddress) -> Result<TxHash>;

    async fn update_bridge(&mut self, _index: usize, _enable_bridge_to: bool, _enable_bridge_from: bool) -> Result<TxHash>;

    async fn is_bridge_to_enabled(&self, _bridge: ContractAddress) -> Result<bool>;

    async fn is_bridge_from_enabled(&self, _bridge: ContractAddress) -> Result<bool>;
}

#[async_trait]
impl<T: Callable + Sync + Send> ZkLink for T {
    async fn deposit_erc20(&mut self,
                           _token: ContractAddress,
                           _amount: u128,
                           _zk_link_address: ContractAddress,
                           _sub_account_id: u8,
                           _mapping: bool)
                           -> Result<TxHash> {
        self.invoke("depositERC20", (_token, _amount, _zk_link_address, _sub_account_id, _mapping))
            .await
    }

    async fn transfer_erc20(&mut self,
                            _token: ContractAddress,
                            _to: ContractAddress,
                            _amount: u128,
                            _max_amount: u128,
                            _is_standard: bool)
                            -> Result<TxHash> {
        self.invoke("transferERC20", (_token, _to, _amount, _max_amount, _is_standard)).await
    }

    async fn request_full_exit(&mut self, _account_id: u32, _sub_account_id: u8, _token_id: u16, _mapping: bool) -> Result<TxHash> {
        self.invoke("requestFullExit", (_account_id, _sub_account_id, _token_id, _mapping)).await
    }

    async fn activate_exodus_mode(&mut self) -> Result<TxHash> {
        self.invoke("activateExodusMode", ()).await
    }

    async fn perform_exodus(&mut self,
                            _stored_block_info: StoredBlockInfo,
                            _owner: ContractAddress,
                            _account_id: u32,
                            _sub_account_id: u8,
                            _withdraw_token_id: u16,
                            _deduct_token_id: u16,
                            _amount: u128,
                            _proof: Vec<U256>)
                            -> Result<TxHash> {
        self.invoke("performExodus",
                    (_stored_block_info, _owner, _account_id, _sub_account_id, _withdraw_token_id, _deduct_token_id, _amount, _proof))
            .await
    }

    async fn set_auth_pubkey_hash(&mut self, _pubkey_hash: Felt252, _nonce: u32) -> Result<TxHash> {
        self.invoke("setAuthPubkeyHash", (_pubkey_hash, _nonce)).await
    }

    async fn withdraw_pending_balance(&mut self, _owner: ContractAddress, _token_id: u16, _amount: u128) -> Result<TxHash> {
        self.invoke("withdrawPendingBalance", (_owner, _token_id, _amount)).await
    }

    async fn get_pending_balance(&self, _address: ContractAddress, _token_id: u16) -> Result<u128> {
        let ret = self.call("getPendingBalance", (_address, _token_id)).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    // =================Validator interface=================
    async fn commit_blocks(&mut self, _last_committed_block_data: StoredBlockInfo, _new_blocks_data: Vec<CommitBlockInfo>) -> Result<TxHash> {
        self.invoke("commitBlocks", (_last_committed_block_data, _new_blocks_data)).await
    }
    /// Blocks commitment verification.
    /// Only verifies block commitments without any other processing
    async fn commit_compressed_blocks(&mut self,
                                      _last_committed_block_data: StoredBlockInfo,
                                      _new_blocks_data: Vec<CommitBlockInfo>,
                                      _new_blocks_extra_data: Vec<CompressedBlockExtraInfo>)
                                      -> Result<TxHash> {
        self.invoke("commitCompressedBlocks",
                    (_last_committed_block_data, _new_blocks_data, _new_blocks_extra_data))
            .await
    }

    async fn execute_blocks(&mut self, _blocks_data: Vec<ExecuteBlockInfo>) -> Result<TxHash> {
        self.invoke("executeBlocks", _blocks_data).await
    }

    // =================Block interface=====================
    async fn prove_blocks(&mut self, _committed_blocks: Vec<StoredBlockInfo>, _proof: ProofInput) -> Result<TxHash> {
        self.invoke("proveBlocks", (_committed_blocks, _proof)).await
    }

    async fn revert_blocks(&mut self, _blocks_to_revert: Vec<StoredBlockInfo>) -> Result<TxHash> {
        self.invoke("revertBlocks", _blocks_to_revert).await
    }

    // =================Cross chain block synchronization===============
    async fn receive_synchronization_progress(&mut self, _sync_hash: U256, _progress: U256) -> Result<TxHash> {
        self.invoke("receiveSynchronizationProgress", (_sync_hash, _progress)).await
    }

    async fn get_synchronized_progress(&self, _block: StoredBlockInfo) -> Result<U256> {
        let ret = self.call("getSynchronizedProgress", _block).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn sync_blocks(&mut self, _block: StoredBlockInfo) -> Result<TxHash> {
        self.invoke("syncBlocks", _block).await
    }

    // =================Fast withdraw and Accept===============
    async fn accept_erc20(&mut self,
                          _accepter: ContractAddress,
                          _account_id: u32,
                          _receiver: ContractAddress,
                          _token_id: u16,
                          _amount: u128,
                          _withdraw_fee_rate: u16,
                          _nonce: u32,
                          _amount_transfer: u128)
                          -> Result<TxHash> {
        self.invoke("acceptERC20",
                    (_accepter, _account_id, _receiver, _token_id, _amount, _withdraw_fee_rate, _nonce, _amount_transfer))
            .await
    }

    async fn broker_allowance(&self, _token_id: u16, _accepter: ContractAddress, _broker: ContractAddress) -> Result<u128> {
        let ret = self.call("brokerAllowance", (_token_id, _accepter, _broker)).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn broker_approve(&mut self, _token_id: u16, _broker: ContractAddress, _amount: u128) -> Result<TxHash> {
        self.invoke("brokerApprove", (_token_id, _broker, _amount)).await
    }

    // =================Governance interface===============
    async fn change_governor(&mut self, _new_governor: ContractAddress) -> Result<TxHash> {
        self.invoke("changeGovernor", _new_governor).await
    }

    async fn add_token(&mut self, _token_id: u16, _token_address: ContractAddress, _decimals: u8, _standard: bool) -> Result<TxHash> {
        self.invoke("addToken", (_token_id, _token_address, _decimals, _standard)).await
    }

    async fn add_tokens(&mut self, _token_list: Vec<Token>) -> Result<TxHash> {
        self.invoke("addTokens", _token_list).await
    }

    async fn set_token_paused(&mut self, _token_id: u16, _token_paused: bool) -> Result<TxHash> {
        self.invoke("setTokenPaused", (_token_id, _token_paused)).await
    }

    async fn set_validator(&mut self, _validator: ContractAddress, _active: bool) -> Result<TxHash> {
        self.invoke("setValidator", _validator).await
    }

    async fn add_bridge(&mut self, _bridge: ContractAddress) -> Result<TxHash> {
        self.invoke("addBridge", _bridge).await
    }

    async fn update_bridge(&mut self, _index: usize, _enable_bridge_to: bool, _enable_bridge_from: bool) -> Result<TxHash> {
        self.invoke("updateBridge", (_index, _enable_bridge_to, _enable_bridge_from)).await
    }

    async fn is_bridge_to_enabled(&self, _bridge: ContractAddress) -> Result<bool> {
        let ret = self.call("isBridgeToEnabled", _bridge).await?;
        Ok(from_slice(ret.as_slice())?)
    }

    async fn is_bridge_from_enabled(&self, _bridge: ContractAddress) -> Result<bool> {
        let ret = self.call("isBridgeFromEnabled", _bridge).await?;
        Ok(from_slice(ret.as_slice())?)
    }
}
