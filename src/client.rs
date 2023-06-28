use std::sync::Arc;
use crate::proto::TxHash;
use crate::to_field_elements;
use serde::Serialize;
use starknet::accounts::{Account, Call, ConnectedAccount, SingleOwnerAccount};
use starknet::core::types::{BlockId, BlockTag, BroadcastedDeclareTransaction, BroadcastedDeclareTransactionV1, BroadcastedDeclareTransactionV2, BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV1, CompressedLegacyContractClass, FieldElement };
use starknet::core::utils::get_selector_from_name;
use starknet::providers::sequencer::models::{TransactionRequest, DeclareTransactionRequest};
use starknet::providers::{Provider, SequencerGatewayProvider};
use starknet::signers::{LocalWallet, SigningKey};

pub struct StarkClient {
    inner: SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>,
    pub contract_address: FieldElement,
    pub address: FieldElement,
    pub local_wallet: LocalWallet,
}

impl StarkClient {
    pub fn new(
        web3_url: &str,
        private_key_hex: &str,
        address: &str,
        contract_address: &str,
        chain_id: FieldElement,
    ) -> Self {
        let gateway_url: url::Url = format!("{}/gateway", web3_url).parse().unwrap();
        let feeder_gateway_url: url::Url = format!("{}/feeder_gateway", web3_url).parse().unwrap();
        let provider = SequencerGatewayProvider::new(gateway_url, feeder_gateway_url, chain_id);
        let signer = LocalWallet::from(SigningKey::from_secret_scalar(
            FieldElement::from_hex_be(private_key_hex).unwrap(),
        ));
        let address = FieldElement::from_hex_be(address).unwrap();
        let contract_address = FieldElement::from_hex_be(contract_address).unwrap();
        let account = SingleOwnerAccount::new(provider, signer.clone(), address, chain_id);
        Self {
            inner: account,
            contract_address,
            address,
            local_wallet: signer,
        }
    }

    pub fn client(&self) -> &SingleOwnerAccount<SequencerGatewayProvider, LocalWallet> {
        &self.inner
    }

    pub async fn get_last_block_number(&self) -> anyhow::Result<u64> {
        let number = self.client().provider().block_number().await?;
        Ok(number)
    }

    pub async fn get_pending_nonce(&self) -> anyhow::Result<FieldElement> {
        let nonce = Provider::get_nonce(
            self.client().provider(),
            BlockId::Tag(BlockTag::Pending),
            self.contract_address,
        )
        .await?;
        Ok(nonce)
    }

    pub async fn send_transaction(
        &self,
        request: TransactionRequest,
    ) -> anyhow::Result<FieldElement> {
        let tx_hash = match request {
            TransactionRequest::Declare(value) => {
                match value {
                    DeclareTransactionRequest::V1(tx) => {
                        let abi = tx.contract_class.abi.clone().map(|abi_list|{
                            abi_list.into_iter().map(|abi| abi.into()).collect()
                        });
                        let contract_class = CompressedLegacyContractClass {
                            program: tx.contract_class.program.clone(),
                            entry_points_by_type: tx.contract_class.entry_points_by_type.clone().into(),
                            abi,
                        };
                        let t = BroadcastedDeclareTransactionV1 {
                         max_fee: tx.max_fee,
                            signature: tx.signature,
                            nonce: tx.nonce,
                            contract_class: Arc::new(contract_class),
                            sender_address: tx.sender_address,
                        };
                        let tx = BroadcastedDeclareTransaction::V1(t);
                        let result = self.client().provider().add_declare_transaction(tx).await?;
                        result.transaction_hash
                    },
                    DeclareTransactionRequest::V2(tx) => {
                        let contract_class = todo!("get flatten contract class");
                        let t = BroadcastedDeclareTransactionV2 {
                            max_fee: tx.max_fee,
                            signature: tx.signature,
                            nonce: tx.nonce,
                            contract_class: Arc::new(contract_class),
                            sender_address: tx.sender_address,
                            compiled_class_hash: tx.compiled_class_hash,
                        };
                        let tx = BroadcastedDeclareTransaction::V2(t);
                        let result = self.client().provider().add_declare_transaction(tx).await?;
                        result.transaction_hash
                    }
                }
            }
            TransactionRequest::DeployAccount(value) => {
                let tx = BroadcastedDeployAccountTransaction {
                    class_hash: value.class_hash,
                    contract_address_salt: value.contract_address_salt,
                    constructor_calldata: value.constructor_calldata,
                    max_fee: value.max_fee,
                    signature: value.signature,
                    nonce: value.nonce
                };
                let r = self
                    .client()
                    .provider()
                    .add_deploy_account_transaction(tx)
                    .await?;
                r.transaction_hash
            }
            TransactionRequest::InvokeFunction(value) => {
                let tx_v1 = BroadcastedInvokeTransactionV1 {
                    max_fee: value.max_fee,
                    signature: value.signature,
                    nonce: value.nonce,
                    sender_address: value.sender_address,
                    calldata: value.calldata
                };
                let tx = BroadcastedInvokeTransaction::V1(tx_v1);
                let r = self.client().provider().add_invoke_transaction(tx).await?;
                r.transaction_hash
            }
        };
        Ok(tx_hash)
    }

    // pub async fn get_transaction_receipt(&self, tx_hash: TxHash) -> anyhow::Result<FieldElement> {
    //     let r = Provider::get_transaction_receipt(self.client().provider(), &tx_hash).await?;
    //     match r {
    //         MaybePendingTransactionReceipt::Receipt(receipt) => {
    //             match receipt {
    //                 TransactionReceipt::DeployAccount(recipt) => {
    //                     let s = recipt.status
    //                     Ok(receipt.transaction_hash)
    //                 },
    //                 TransactionReceipt::Declare(d) => Ok(d.transaction_hash)
    //                 TransactionReceipt::
    //             }
    //         },
    //         MaybePendingTransactionReceipt::PendingReceipt(receipt) => {
    //
    //         }
    //     }
    //
    // }

    pub async fn call<T>(&self, func_name: &str, calldata: T) -> anyhow::Result<TxHash>
    where
        T: Serialize,
    {
        let selector = get_selector_from_name(func_name).unwrap();
        let calldata_elements = to_field_elements(calldata)?;
        let result = self
            .client()
            .execute(vec![Call {
                to: self.contract_address,
                selector,
                calldata: calldata_elements,
            }])
            .send()
            .await?;
        let tx_hash = result.transaction_hash;
        Ok(tx_hash.into())
    }
}

#[cfg(test)]
mod test {
    use crate::client::StarkClient;
    use crate::num::PrimitiveU256;
    use crate::proto::{
        Bytes, CommitBlockInfo, CompressedBlockExtraInfo, ExecuteBlockInfo, OnchainOperationData,
        StoredBlockInfo, TxHash,
    };
    use crate::U256;
    use starknet::core::chain_id;

    impl StarkClient {
        pub async fn test_u128_list(
            &self,
            calldata: Vec<u128>,
            index: usize,
        ) -> anyhow::Result<TxHash> {
            self.call("u128Test", (calldata, index)).await
        }
        pub async fn test_u256_list(
            &self,
            calldata: Vec<U256>,
            index: usize,
        ) -> anyhow::Result<TxHash> {
            self.call("u256sTest", (calldata, index)).await
        }

        pub async fn test_u8_array(&self, calldata: Vec<u8>) -> anyhow::Result<TxHash> {
            self.call("u8sTest1", calldata).await
        }

        pub async fn test_bytes_list(
            &self,
            calldata: Vec<Bytes>,
            index: usize,
        ) -> anyhow::Result<TxHash> {
            self.call("bytesListTest", (calldata, index)).await
        }

        pub async fn test_stored_block_info(
            &self,
            info_list: Vec<StoredBlockInfo>,
            i: usize,
        ) -> anyhow::Result<TxHash> {
            self.call("StoredBlockInfoTest", (info_list, i)).await
        }

        pub async fn test_commit_block_info(
            &self,
            info_list: Vec<CommitBlockInfo>,
            i: usize,
            j: usize,
        ) -> anyhow::Result<TxHash> {
            self.call("CommitBlockInfoTest", (info_list, i, j)).await
        }

        pub async fn test_compressed_block_extra_info(
            &self,
            info_list: Vec<CompressedBlockExtraInfo>,
            i: usize,
            j: usize,
        ) -> anyhow::Result<TxHash> {
            self.call("CompressedBlockExtraInfoTest", (info_list, i, j))
                .await
        }

        pub async fn test_execute_block_info(
            &self,
            info_list: Vec<ExecuteBlockInfo>,
            i: usize,
            j: u8,
            op_type: u8,
        ) -> anyhow::Result<TxHash> {
            self.call("ExecuteBlockInfoTest", (info_list, i, j, op_type))
                .await
        }

        pub async fn test_execute_block_info2(
            &self,
            info_list: Vec<ExecuteBlockInfo>,
            i: usize,
            j: u8,
            op_type: u8,
        ) -> anyhow::Result<TxHash> {
            self.call("ExecuteBlockInfoTest2", (info_list, i, j, op_type))
                .await
        }
    }

    fn client() -> StarkClient {
        let web3_url = "http://127.0.0.1:5050";
        let private_key_hex = "6fb84183efc4de5a4707ac7ad487d5e1db4ec34a2c1500ee25fe6ab29940462";
        let address = "0x13528b84b5a4ed4a7aff3b3a27363565f38608499f1404f73e15c11fce9aa5d";
        let contract_address = "0x474c2b5858139a7d7f20e71f836fc98f130c2c2992888433fbdce742a95d564";
        let chain_id = chain_id::TESTNET;
        let client = StarkClient::new(
            web3_url,
            private_key_hex,
            address,
            contract_address,
            chain_id,
        );
        client
    }

    #[tokio::test]
    async fn test_u128_list() {
        let r = client().test_u128_list(vec![1u128], 0).await;
        println!("{:?}", r);
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_u256_list() {
        let r = client()
            .test_u256_list(vec![PrimitiveU256::from(1u8).into()], 0)
            .await;
        println!("{:?}", r);
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_u8_array() {
        let r = client()
            .test_u8_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
            .await;
        println!("{:?}", r);
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_bytes_list() {
        let b1 = Bytes {
            size: 1,
            data: vec![1, 2, 3],
        };
        let b2 = Bytes::default();
        let r = client().test_bytes_list(vec![b1, b2], 1).await;
        println!("{r:?}");
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_stored_block_info() {
        let info = StoredBlockInfo {
            block_number: 1,
            priority_operations: 1,
            pending_onchain_operations_hash: PrimitiveU256::from(2).into(),
            timestamp: 3,
            state_hash: PrimitiveU256::from(4u8).into(),
            commitment: PrimitiveU256::from(5u8).into(),
            sync_hash: PrimitiveU256::from(6u8).into(),
        };
        let r = client().test_stored_block_info(vec![info], 0).await;
        println!("{r:?}");
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_compressed_block_extra_info() {
        let info = CompressedBlockExtraInfo {
            public_data_hash: PrimitiveU256::from(1u8).into(),
            offset_commitment_hash: PrimitiveU256::from(2u8).into(),
            onchain_operation_pubdata_hashs: vec![
                PrimitiveU256::from(1u8).into(),
                PrimitiveU256::from(2u8).into(),
            ],
        };
        let r = client()
            .test_compressed_block_extra_info(vec![info], 0, 0)
            .await;
        println!("{:?}", r);
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_commit_block_info() {
        let op = OnchainOperationData {
            eth_witness: Bytes::default(),
            public_data_offset: 2,
        };
        let info = CommitBlockInfo {
            new_state_hash: PrimitiveU256::from(1u8).into(),
            public_data: Bytes::default(),
            timestamp: 2,
            onchain_operations: vec![op],
            block_number: 3,
            fee_account: 4,
        };
        println!("{info:?}");
        let r = client().test_commit_block_info(vec![info], 0, 0).await;
        println!("{:?}", r);
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_execute_block_info() {
        let stored_block_info = StoredBlockInfo {
            block_number: 1,
            priority_operations: 2,
            pending_onchain_operations_hash: PrimitiveU256::from(3u8).into(),
            timestamp: 4,
            state_hash: PrimitiveU256::from(5u8).into(),
            commitment: PrimitiveU256::from(6u8).into(),
            sync_hash: PrimitiveU256::from(7u8).into(),
        };

        let r = client()
            .test_stored_block_info(vec![stored_block_info.clone()], 0)
            .await;
        println!("{:?}", r);
        assert!(r.is_ok());

        let info = ExecuteBlockInfo {
            stored_block_info,
            pending_onchain_ops_pubdata: vec![Bytes {
                size: 2,
                data: vec![1, 2],
            }],
        };
        let r = client()
            .test_execute_block_info2(vec![info.clone()], 0, 0, 1)
            .await;
        println!("{:?}", r);
        assert!(r.is_ok());
        let r = client().test_execute_block_info(vec![info], 0, 0, 1).await;
        println!("{r:?}");
        assert!(r.is_ok());
    }
}
