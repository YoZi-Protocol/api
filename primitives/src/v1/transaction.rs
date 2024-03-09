use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use time::OffsetDateTime;

use crate::{
    entities::{AmountValue, BlockState},
    v1::BlockResponse,
};

with_prefix!(prefix_block "block_", &["chain_"]);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TransactionFindRequest {
    pub chain_id: String,
    pub block: Option<String>,
    pub address: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize, Deserialize)]
#[builder(default, setter(into, strip_option, prefix = "with"))]
pub struct TransactionResponse {
    #[serde(flatten, with = "prefix_block")]
    block: BlockOrChainId,
    index: Option<i64>,
    hash: String,
    from_address: String,
    to_address: String,
    value: Option<AmountValue>,
}

impl TransactionResponse {
    pub fn builder() -> TransactionResponseBuilder {
        TransactionResponseBuilder::default()
    }

    pub fn block(&self) -> Option<&BlockResponse> {
        match &self.block {
            BlockOrChainId::Block(block) => Some(block),
            BlockOrChainId::ChainId(_) => None,
        }
    }

    pub fn block_number(&self) -> Option<i64> {
        self.block().map(|t| t.number())
    }

    pub fn block_hash(&self) -> Option<&str> {
        self.block().map(|t| t.hash())
    }

    pub fn block_state(&self) -> BlockState {
        self.block().map(|t| t.state()).unwrap_or_default()
    }

    pub fn block_finalized(&self) -> Option<bool> {
        self.block().map(|t| t.finalized())
    }

    pub fn block_finalized_at(&self) -> Option<OffsetDateTime> {
        self.block().and_then(|t| t.finalized_at().copied())
    }

    pub fn chain_id(&self) -> &str {
        match &self.block {
            BlockOrChainId::Block(block) => block.chain_id(),
            BlockOrChainId::ChainId(chain_id) => chain_id,
        }
    }

    pub fn index(&self) -> Option<i64> {
        self.index
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn from_address(&self) -> &str {
        &self.from_address
    }

    pub fn to_address(&self) -> &str {
        &self.to_address
    }

    pub fn value(&self) -> Option<&AmountValue> {
        self.value.as_ref()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum BlockOrChainId {
    Block(BlockResponse),
    ChainId(String),
}

impl Default for BlockOrChainId {
    fn default() -> Self {
        Self::ChainId(String::new())
    }
}

impl From<BlockResponse> for BlockOrChainId {
    fn from(value: BlockResponse) -> Self {
        Self::Block(value)
    }
}

impl<S: AsRef<str>> From<S> for BlockOrChainId {
    fn from(value: S) -> Self {
        Self::ChainId(value.as_ref().to_owned())
    }
}
