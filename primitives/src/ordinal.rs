use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::Lowercase, hex::Hex, serde_as, skip_serializing_none};

use crate::entities::{ContractType, ExtrinsicOperation};

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize, Deserialize)]
#[builder(default, setter(into, strip_option, prefix = "with"))]
pub struct Ordinal {
    #[serde(rename = "p")]
    pub protocol: ContractType,
    pub tick: String,
    pub name: Option<String>,
    #[serde(rename = "op")]
    pub operation: ExtrinsicOperation,
    #[serde(flatten)]
    pub operand: Option<SingleOrBatch>,
    #[serde(rename = "max")]
    pub max_supply: Option<String>,
    #[serde(rename = "lim")]
    pub mint_limit: Option<String>,
    pub uri: Option<String>,
    #[serde(rename = "nbf")]
    pub not_before: Option<String>,
    pub period: Option<String>,
    #[serde(rename = "exp")]
    pub expires_at: Option<String>,
    pub a: Option<A>,
    pub proof: Option<Proof>,
}

impl Ordinal {
    pub fn builder() -> OrdinalBuilder {
        OrdinalBuilder::default()
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Operand {
    pub id: Option<String>,
    #[serde(rename = "amt")]
    pub amount: Option<String>,
    pub to: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SingleOrBatch {
    #[serde(rename = "to")]
    Batch(Vec<Operand>),
    #[serde(untagged)]
    Single(Operand),
}

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct A(
    #[serde_as(as = "Hex<Lowercase>")] pub Vec<u8>,
    #[serde_as(as = "Hex<Lowercase>")] pub Vec<u8>,
);

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Proof(
    #[serde_as(as = "[Hex<Lowercase>; 2]")] pub [Vec<u8>; 2],
    #[serde_as(as = "[[Hex<Lowercase>; 2]; 2]")] pub [[Vec<u8>; 2]; 2],
    #[serde_as(as = "[Hex<Lowercase>; 2]")] pub [Vec<u8>; 2],
);
