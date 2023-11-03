use dharitri_sc::api::ManagedTypeApi;
use dharitri_sc::types::ManagedVec;
use transaction::transaction_status::TransactionStatus;
use transaction::EthTransaction;

dharitri_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum Action<M: ManagedTypeApi> {
    Nothing,
    SetCurrentTransactionBatchStatus {
        dct_safe_batch_id: u64,
        tx_batch_status: ManagedVec<M, TransactionStatus>,
    },
    BatchTransferDctToken {
        eth_batch_id: u64,
        transfers: ManagedVec<M, EthTransaction<M>>,
    },
}

impl<M: ManagedTypeApi> Action<M> {
    /// Only pending actions are kept in storage,
    /// both executed and discarded actions are removed (converted to `Nothing`).
    /// So this is equivalent to `action != Action::Nothing`.
    pub fn is_pending(&self) -> bool {
        !matches!(*self, Action::Nothing)
    }
}
