use std::sync::Arc;

use sea_orm::{
    sea_query::IntoCondition, ColumnTrait as _, DatabaseConnection, EntityTrait as _,
    IntoSimpleExpr, Order, QueryFilter as _, QueryOrder as _, QuerySelect as _,
};

use eos420_service_derive::cache;

use crate::{
    entities,
    managers::{ClassManager, ContractManager, TransactionManager},
    primitives::{v1::ExtrinsicResponse, Uint256},
    utilities::calculate_amount,
    CacheService, IdService,
};

#[di::injectable]
#[derive(Clone)]
pub struct ExtrinsicManager {
    pub cache: Arc<CacheService<entities::extrinsic::Model>>,
    pub db: Arc<DatabaseConnection>,
    pub id: Arc<IdService>,
    pub class_manager: Arc<ClassManager>,
    pub contract_manager: Arc<ContractManager>,
    pub transaction_manager: Arc<TransactionManager>,
}

impl ExtrinsicManager {
    #[cache]
    pub async fn find(
        &self,
        chain_id: &str,
        tx_hash: &str,
        index: i64,
    ) -> Option<entities::extrinsic::Model> {
        entities::extrinsic::Entity::find()
            .filter(entities::extrinsic::Column::ChainId.eq(chain_id))
            .filter(entities::extrinsic::Column::TxHash.eq(tx_hash))
            .filter(entities::extrinsic::Column::Index.eq(index))
            .order_by_desc(entities::extrinsic::Column::Id)
            .one(self.db.as_ref())
            .await
            .ok()?
    }

    pub async fn query<C: IntoSimpleExpr, F: IntoCondition>(
        &self,
        filter: Vec<F>,
        order: Vec<(C, Order)>,
        limit: Option<u64>,
    ) -> Vec<entities::extrinsic::Model> {
        let mut query = entities::extrinsic::Entity::find();

        for f in filter {
            query = query.filter(f);
        }

        for (c, o) in order {
            query = query.order_by(c, o);
        }

        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        query.all(self.db.as_ref()).await.unwrap_or_default()
    }

    pub async fn dump(
        &self,
        extrinsic: &entities::extrinsic::Model,
        _agony: bool,
    ) -> Option<ExtrinsicResponse> {
        let contract = self
            .contract_manager
            .find(&extrinsic.chain_id, &extrinsic.asset_id)
            .await?;

        let metadata = self.contract_manager.dump(&contract, false).await?;

        let transaction = self
            .transaction_manager
            .find(&extrinsic.chain_id, &extrinsic.tx_hash)
            .await?;

        let transaction = self.transaction_manager.dump(&transaction, false).await?;

        let mut response = ExtrinsicResponse::builder();
        response
            .with_transaction(transaction)
            .with_contract(metadata)
            .with_index(extrinsic.index)
            .with_from_address(&extrinsic.from_address)
            .with_to_address(&extrinsic.to_address)
            .with_operation(extrinsic.operation)
            .with_state(extrinsic.state);

        if let Some(drop_reason) = extrinsic.drop_reason {
            response.with_drop_reason(drop_reason);
        }

        if let Some(decimals) = contract.decimals {
            let amount = Uint256::from_str_prefixed(&extrinsic.value).unwrap_or_default();
            let amount = calculate_amount(&amount, decimals);
            response.with_amount(amount);
        } else {
            response.with_identifier(&extrinsic.value);
        }

        response.build().ok()
    }
}
