use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use time::OffsetDateTime;

use crate::{
    entities::{AmountValue, BlockState, ClassType, ContractType, DropReason, ExtrinsicOperation},
    v1::{ContractResponse, TransactionResponse},
};

with_prefix!(prefix_tx "tx_", &["block_"]);
with_prefix!(prefix_asset "asset_", &["chain_"]);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ExtrinsicFindRequest {
    pub chain_id: String,
    pub block: Option<String>,
    pub tx_hash: Option<String>,
    pub asset_id: Option<Vec<String>>,
    pub address: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize, Deserialize)]
#[builder(default, setter(into, strip_option, prefix = "with"))]
pub struct ExtrinsicResponse {
    #[serde(flatten, with = "prefix_tx")]
    transaction: TransactionResponse,
    #[serde(flatten, with = "prefix_asset")]
    contract: ContractResponse,
    index: i64,
    from_address: String,
    to_address: String,
    operation: ExtrinsicOperation,
    state: BlockState,
    drop_reason: Option<DropReason>,
    // amount is 0x hex string or string
    amount: Option<AmountValue>,
    // identifier is hex string without 0x prefix or string
    identifier: Option<String>,
}

impl ExtrinsicResponse {
    pub fn builder() -> ExtrinsicResponseBuilder {
        ExtrinsicResponseBuilder::default()
    }

    pub fn transaction(&self) -> &TransactionResponse {
        &self.transaction
    }

    pub fn chain_id(&self) -> &str {
        self.transaction().chain_id()
    }

    pub fn block_number(&self) -> Option<i64> {
        self.transaction().block_number()
    }

    pub fn block_hash(&self) -> Option<&str> {
        self.transaction().block_hash()
    }

    pub fn tx_index(&self) -> Option<i64> {
        self.transaction().index()
    }

    pub fn tx_hash(&self) -> &str {
        self.transaction().hash()
    }

    pub fn block_state(&self) -> BlockState {
        self.transaction().block_state()
    }

    pub fn block_finalized(&self) -> Option<bool> {
        self.transaction().block_finalized()
    }

    pub fn block_finalized_at(&self) -> Option<OffsetDateTime> {
        self.transaction().block_finalized_at()
    }

    pub fn contract(&self) -> &ContractResponse {
        &self.contract
    }

    pub fn asset_id(&self) -> &str {
        self.contract().id()
    }

    pub fn name(&self) -> &str {
        self.contract().name()
    }

    pub fn symbol(&self) -> &str {
        self.contract().symbol()
    }

    pub fn r#type(&self) -> ClassType {
        self.contract().r#type()
    }

    pub fn protocol(&self) -> ContractType {
        self.contract().protocol()
    }

    pub fn decimals(&self) -> Option<i32> {
        self.contract().decimals()
    }

    pub fn index(&self) -> i64 {
        self.index
    }

    pub fn from_address(&self) -> &str {
        &self.from_address
    }

    pub fn to_address(&self) -> &str {
        &self.to_address
    }

    pub fn amount(&self) -> Option<AmountValue> {
        self.amount.clone()
    }

    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }
}
