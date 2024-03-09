use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use time::{serde::rfc3339, OffsetDateTime};

use crate::entities::BlockState;

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize, Deserialize)]
#[builder(default, setter(into, strip_option, prefix = "with"))]
pub struct BlockResponse {
    chain_id: String,
    number: i64,
    hash: String,
    state: BlockState,
    transaction_count: Option<i64>,
    extrinsic_count: Option<i64>,
    finalized: bool,
    #[serde(with = "rfc3339::option")]
    finalized_at: Option<OffsetDateTime>,
}

impl BlockResponse {
    pub fn builder() -> BlockResponseBuilder {
        BlockResponseBuilder::default()
    }

    pub fn chain_id(&self) -> &str {
        &self.chain_id
    }

    pub fn number(&self) -> i64 {
        self.number
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn state(&self) -> BlockState {
        self.state
    }

    pub fn transaction_count(&self) -> i64 {
        self.transaction_count.unwrap_or_default()
    }

    pub fn extrinsic_count(&self) -> i64 {
        self.extrinsic_count.unwrap_or_default()
    }

    pub fn finalized(&self) -> bool {
        self.finalized
    }

    pub fn finalized_at(&self) -> Option<&OffsetDateTime> {
        self.finalized_at.as_ref()
    }
}
