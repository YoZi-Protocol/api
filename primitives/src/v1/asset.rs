use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    entities::{AmountValue, ClassType, ContractType},
    v1::ContractResponse,
};

with_prefix!(prefix_asset "asset_", &["chain_"]);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetFindRequest {
    pub chain_id: Option<String>,
    pub address: Option<String>,
    pub asset_id: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize, Deserialize)]
#[builder(default, setter(into, strip_option, prefix = "with"))]
pub struct AssetResponse {
    #[serde(flatten, with = "prefix_asset")]
    contract: ContractResponse,
    tx_hash: Option<String>,
    // amount is 0x hex string or string
    amount: Option<AmountValue>,
    // identifier is hex string without 0x prefix or string
    identifier: Option<String>,
    locked: Option<BoolOrAmount>,
}

impl AssetResponse {
    pub fn builder() -> AssetResponseBuilder {
        AssetResponseBuilder::default()
    }

    pub fn contract(&self) -> &ContractResponse {
        &self.contract
    }

    pub fn chain_id(&self) -> &str {
        self.contract().chain_id()
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

    pub fn amount(&self) -> Option<AmountValue> {
        self.amount.clone()
    }

    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum BoolOrAmount {
    Bool(bool),
    Amount(AmountValue),
}

impl From<bool> for BoolOrAmount {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl<A: Into<AmountValue>> From<A> for BoolOrAmount {
    fn from(value: A) -> Self {
        Self::Amount(value.into())
    }
}
