use async_trait::async_trait;
use starknet_api::core::ContractAddress;

use crate::client::StarkClient;
use crate::contract::Contract;

use crate::primitive::*;
use crate::proto::*;
use crate::U256;

#[async_trait]
pub trait ZkLink {
    // =================User interface=================
    async fn deposit_erc20(
        &mut self,
        _token: ContractAddress,
        _amount: u128,
        _zk_link_address: ContractAddress,
        _sub_account_id: u8,
        _mapping: bool,
    ) {
        unimplemented!("method not allowed")
    }

    async fn transfer_erc20(
        &mut self,
        _token: ContractAddress,
        _to: ContractAddress,
        _amount: u128,
        _max_amount: u128,
        _is_standard: bool,
    ) -> anyhow::Result<u128> {
        unimplemented!("method not allowed")
    }

    async fn request_full_exit(
        &mut self,
        _account_id: u32,
        _sub_account_id: u8,
        _token_id: u16,
        _mapping: bool,
    ) {
        unimplemented!("method not allowed")
    }

    async fn activate_exodus_mode(&mut self) {
        unimplemented!("method not allowed")
    }

    async fn perform_exodus(
        &mut self,
        _stored_block_info: StoredBlockInfo,
        _owner: ContractAddress,
        _account_id: u32,
        _sub_account_id: u8,
        _withdraw_token_id: u16,
        _deduct_token_id: u16,
        _amount: u128,
        _proof: Vec<U256>,
    ) {
        unimplemented!("method not allowed")
    }

    async fn set_auth_pubkey_hash(&mut self, _pubkey_hash: Felt252, _nonce: u32) {
        unimplemented!("method not allowed")
    }

    async fn withdraw_pending_balance(
        &mut self,
        _owner: ContractAddress,
        _token_id: u16,
        _amount: u128,
    ) -> anyhow::Result<u128> {
        unimplemented!("method not allowed")
    }

    async fn get_pending_balance(
        &self,
        _address: ContractAddress,
        _token_id: u16,
    ) -> anyhow::Result<u128> {
        unimplemented!("method not allowed")
    }

    // =================Validator interface=================
    async fn commit_blocks(
        &mut self,
        _last_committed_block_data: StoredBlockInfo,
        _new_blocks_data: Vec<CommitBlockInfo>,
    ) {
        unimplemented!("method not allowed")
    }
    /// Blocks commitment verification.
    /// Only verifies block commitments without any other processing
    async fn commit_compressed_blocks(
        &mut self,
        _last_committed_block_data: StoredBlockInfo,
        _new_blocks_data: Vec<CommitBlockInfo>,
        _new_blocks_extra_data: Vec<CompressedBlockExtraInfo>,
    ) {
        unimplemented!("method not allowed")
    }

    async fn execute_blocks(&mut self, _blocks_data: Vec<ExecuteBlockInfo>) {
        unimplemented!("method not allowed")
    }

    // =================Block interface=====================
    async fn prove_blocks(&mut self, _committed_blocks: Vec<StoredBlockInfo>, _proof: ProofInput) {
        unimplemented!("method not allowed")
    }

    async fn revert_blocks(&mut self, _blocks_to_revert: Vec<StoredBlockInfo>) {
        unimplemented!("method not allowed")
    }

    // =================Cross chain block synchronization===============
    async fn receive_synchronization_progress(&mut self, _sync_hash: U256, _progress: U256) {
        unimplemented!("method not allowed")
    }

    async fn get_synchronized_progress(&self, _block: StoredBlockInfo) -> anyhow::Result<U256> {
        unimplemented!("method not allowed")
    }

    async fn sync_blocks(&mut self, _block: StoredBlockInfo) {
        unimplemented!("method not allowed")
    }

    // =================Fast withdraw and Accept===============
    async fn accept_erc20(
        &mut self,
        _accepter: ContractAddress,
        _account_id: u32,
        _receiver: ContractAddress,
        _token_id: u16,
        _amount: u128,
        _withdraw_fee_rate: u16,
        _nonce: u32,
        _amount_transfer: u128,
    ) {
        unimplemented!("method not allowed")
    }

    async fn broker_allowance(
        &self,
        _token_id: u16,
        _accepter: ContractAddress,
        _broker: ContractAddress,
    ) -> anyhow::Result<u128> {
        unimplemented!("method not allowed")
    }

    async fn broker_approve(
        &mut self,
        _token_id: u16,
        _broker: ContractAddress,
        _amount: u128,
    ) -> anyhow::Result<bool> {
        unimplemented!("method not allowed")
    }

    // =================Governance interface===============
    async fn change_governor(&mut self, _new_governor: ContractAddress) {
        unimplemented!("method not allowed")
    }

    async fn add_token(
        &mut self,
        _token_id: u16,
        _token_address: ContractAddress,
        _decimals: u8,
        _standard: bool,
    ) {
        unimplemented!("method not allowed")
    }

    async fn add_tokens(&mut self, _token_list: Vec<Token>) {
        unimplemented!("method not allowed")
    }

    async fn set_token_paused(&mut self, _token_id: u16, _token_paused: bool) {
        unimplemented!("method not allowed")
    }

    async fn set_validator(&mut self, _validator: ContractAddress, _active: bool) {
        unimplemented!("method not allowed")
    }

    async fn add_bridge(&mut self, _bridge: ContractAddress) -> anyhow::Result<usize> {
        unimplemented!("method not allowed")
    }

    async fn update_bridge(
        &mut self,
        _index: usize,
        _enable_bridge_to: bool,
        _enable_bridge_from: bool,
    ) {
        unimplemented!("method not allowed")
    }

    async fn is_bridge_to_enabled(&self, _bridge: ContractAddress) -> anyhow::Result<bool> {
        unimplemented!("method not allowed")
    }

    async fn is_bridge_from_enabled(&self, _bridge: ContractAddress) -> anyhow::Result<bool> {
        unimplemented!("method not allowed")
    }

    // =============Test Interface=============
    // TODO only test
    async fn stored_block_info_test(
        &self,
        _blocks_data: Vec<StoredBlockInfo>,
        i: usize,
    ) -> anyhow::Result<u64> {
        unimplemented!("method not allowed")
    }

    async fn commit_block_info_test(
        &self,
        _blocks_data: Vec<CommitBlockInfo>,
        i: usize,
        j: usize,
    ) -> anyhow::Result<usize> {
        unimplemented!("method not allowed")
    }

    async fn compressed_block_extra_info_test(
        &self,
        _blocks_extra_data: Vec<CompressedBlockExtraInfo>,
        i: usize,
        j: usize,
    ) -> anyhow::Result<U256> {
        unimplemented!("method not allowed")
    }

    async fn execute_block_info_test(
        &self,
        _blocks_data: Vec<ExecuteBlockInfo>,
        i: usize,
        j: usize,
        _op_type: u8,
    ) -> anyhow::Result<u8> {
        unimplemented!("method not allowed")
    }

    async fn u256test(&self, _u256: U256) -> anyhow::Result<(u128, u128)> {
        unimplemented!("method not allowed")
    }

    async fn u256s_test(&self, _u256s: Vec<U256>, i: usize) -> anyhow::Result<(u128, u128)> {
        unimplemented!("method not allowed")
    }

    async fn u8s_test1(&self, _u8s: Vec<u8>) -> anyhow::Result<usize> {
        unimplemented!("method not allowed")
    }

    async fn u8s_test2(&self, _u8s: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        unimplemented!("method not allowed")
    }
}

// TODO
impl Contract for StarkClient {
    type Handler = Box<dyn ZkLink>;

    fn contract(&self) -> Self::Handler {
        unimplemented!("todo")
    }
}
