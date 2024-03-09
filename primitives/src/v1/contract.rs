use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use time::{serde::rfc3339, OffsetDateTime};

use crate::entities::{AmountValue, ClassType, ContractState, ContractType};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ContractFindRequest {
    pub chain_id: Option<String>,
    pub r#type: Option<Vec<ClassType>>,
    pub protocol: Option<Vec<ContractType>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ContractDeployRequest {
    pub protocol: Option<ContractType>,
    pub chain_id: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize, Deserialize)]
#[builder(default, setter(into, strip_option, prefix = "with"))]
pub struct ContractResponse {
    chain_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    r#type: ClassType,
    protocol: ContractType,
    name: String,
    symbol: String,
    description: Option<String>,
    cover_image_uri: Option<String>,
    decimals: Option<i32>,
    max_supply: Option<AmountValue>,
    mint_limit: Option<AmountValue>,
    state: Option<ContractState>,
    not_before: Option<i64>,
    #[serde(with = "rfc3339::option")]
    deployed_at: Option<OffsetDateTime>,
    tx_hash: Option<String>,
    owner: Option<String>,
    to_address: Option<String>,
    fee: Option<AmountValue>,
    supply: Option<AmountValue>,
    holder_count: Option<AmountValue>,
}

impl ContractResponse {
    pub fn builder() -> ContractResponseBuilder {
        ContractResponseBuilder::default()
    }

    pub fn chain_id(&self) -> &str {
        &self.chain_id
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn r#type(&self) -> ClassType {
        self.r#type
    }

    pub fn protocol(&self) -> ContractType {
        self.protocol
    }

    pub fn decimals(&self) -> Option<i32> {
        self.decimals
    }

    pub fn max_supply(&self) -> Option<&AmountValue> {
        self.max_supply.as_ref()
    }

    pub fn mint_limit(&self) -> Option<&AmountValue> {
        self.mint_limit.as_ref()
    }

    pub fn state(&self) -> ContractState {
        self.state.unwrap_or_default()
    }

    pub fn deployed_at(&self) -> Option<&OffsetDateTime> {
        self.deployed_at.as_ref()
    }
}
