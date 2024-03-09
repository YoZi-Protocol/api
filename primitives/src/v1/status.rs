use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StatusRequest {
    pub chain_id: Option<String>,
    pub asset_id: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize, Deserialize)]
#[builder(default, setter(into, strip_option, prefix = "with"))]
pub struct StatusResponse {
    pending: u64,
    finalized: u64,
    dropped: u64,
}

impl StatusResponse {
    pub fn builder() -> StatusResponseBuilder {
        StatusResponseBuilder::default()
    }
}
