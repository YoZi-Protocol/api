use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

use crate::bigint::Uint256;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmountValue {
    U256(Uint256),
    U64(u64),
    F64(f64),
}

impl From<Uint256> for AmountValue {
    fn from(value: Uint256) -> Self {
        Self::U256(value)
    }
}

impl From<u64> for AmountValue {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<f64> for AmountValue {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
#[serde(rename_all = "kebab-case")]
pub enum BlockState {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "indexing")]
    Indexing,
    #[sea_orm(string_value = "confirmed")]
    Confirmed,
    #[sea_orm(string_value = "finalized")]
    Finalized,
    #[sea_orm(string_value = "dropped")]
    Dropped,
}

impl Default for BlockState {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
#[serde(rename_all = "kebab-case")]
pub enum ClassType {
    #[sea_orm(string_value = "fungible")]
    Fungible,
    #[sea_orm(string_value = "non-fungible")]
    NonFungible,
}

impl Default for ClassType {
    fn default() -> Self {
        Self::Fungible
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
#[serde(rename_all = "kebab-case")]
pub enum ContractState {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "deploying")]
    Deploying,
    #[sea_orm(string_value = "deployed")]
    Deployed,
}

impl Default for ContractState {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
#[serde(rename_all = "kebab-case")]
pub enum ContractType {
    #[sea_orm(string_value = "erc20")]
    Erc20,
    #[sea_orm(string_value = "erc721")]
    Erc721,
    #[sea_orm(string_value = "eos20")]
    Eos20,
    #[sea_orm(string_value = "eos420")]
    Eos420,
}

impl Default for ContractType {
    fn default() -> Self {
        Self::Erc20
    }
}

impl From<ContractType> for ClassType {
    fn from(val: ContractType) -> Self {
        match val {
            ContractType::Erc20 | ContractType::Eos20 => ClassType::Fungible,
            ContractType::Erc721 | ContractType::Eos420 => ClassType::NonFungible,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
#[serde(rename_all = "kebab-case")]
pub enum DropReason {
    #[sea_orm(string_value = "unknown")]
    Unknown,
    #[sea_orm(string_value = "context_missing")]
    ContextMissing,
    #[sea_orm(string_value = "context_malformed")]
    ContextMalformed,
    #[sea_orm(string_value = "transaction_dropped")]
    TransactionDropped,
    #[sea_orm(string_value = "transaction_malformed")]
    TransactionMalformed,
    #[sea_orm(string_value = "protocol_mismatch")]
    ProtocolMismatch,
    #[sea_orm(string_value = "operation_invalid")]
    OperationInvalid,
    #[sea_orm(string_value = "operation_unsupported")]
    OperationUnsupported,
    #[sea_orm(string_value = "fee_insufficient")]
    FeeInsufficient,
    #[sea_orm(string_value = "fee_arrearage")]
    FeeArrearage,
    #[sea_orm(string_value = "extrinsic_conflicted")]
    ExtrinsicConflicted,
    #[sea_orm(string_value = "balance_insufficient")]
    BalanceInsufficient,
    #[sea_orm(string_value = "supply_exceeded")]
    SupplyExceeded,
}

impl Default for DropReason {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
#[serde(rename_all = "kebab-case")]
pub enum ExtrinsicOperation {
    #[sea_orm(string_value = "deploy")]
    Deploy,
    #[sea_orm(string_value = "mint")]
    Mint,
    #[sea_orm(string_value = "transfer")]
    Transfer,
    #[sea_orm(string_value = "stake")]
    Stake,
    #[sea_orm(string_value = "burn")]
    Burn,
}

impl Default for ExtrinsicOperation {
    fn default() -> Self {
        Self::Mint
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
#[serde(rename_all = "kebab-case")]
pub enum LockReason {
    #[sea_orm(string_value = "rollup")]
    Rollup,
    #[sea_orm(string_value = "user")]
    User,
}

impl Default for LockReason {
    fn default() -> Self {
        Self::User
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub enum NumberOrHash {
    Number(i64),
    Hash(String),
}

impl Default for NumberOrHash {
    fn default() -> Self {
        Self::Number(1)
    }
}

impl From<i64> for NumberOrHash {
    fn from(value: i64) -> Self {
        Self::Number(value)
    }
}

impl From<&str> for NumberOrHash {
    fn from(value: &str) -> Self {
        Self::Hash(value.to_owned())
    }
}
