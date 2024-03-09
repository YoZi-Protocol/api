use std::sync::Arc;

use sea_orm::{
    sea_query::IntoCondition, ColumnTrait as _, DatabaseConnection, EntityTrait as _,
    IntoSimpleExpr, Order, PaginatorTrait as _, QueryFilter as _, QueryOrder as _,
    QuerySelect as _,
};

use eos420_service_derive::cache;

use crate::{
    entities::{self, NumberOrHash},
    managers::BlockManager,
    primitives::v1::TransactionResponse,
    CacheService, IdService,
};

#[di::injectable]
#[derive(Clone)]
pub struct TransactionManager {
    pub cache: Arc<CacheService<entities::transaction::Model>>,
    pub db: Arc<DatabaseConnection>,
    pub id: Arc<IdService>,
    pub block_manager: Arc<BlockManager>,
}

impl TransactionManager {
    #[cache]
    pub async fn find(
        &self,
        chain_id: &str,
        tx_hash: &str,
    ) -> Option<entities::transaction::Model> {
        entities::transaction::Entity::find()
            .filter(entities::transaction::Column::ChainId.eq(chain_id))
            .filter(entities::transaction::Column::TxHash.eq(tx_hash))
            .order_by_desc(entities::transaction::Column::Id)
            .one(self.db.as_ref())
            .await
            .ok()?
    }

    pub async fn query<C: IntoSimpleExpr, F: IntoCondition>(
        &self,
        filter: Vec<F>,
        order: Vec<(C, Order)>,
        limit: Option<u64>,
    ) -> Vec<entities::transaction::Model> {
        let mut query = entities::transaction::Entity::find();

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

    pub async fn count<F: IntoCondition>(&self, filter: Vec<F>) -> u64 {
        let mut query = entities::transaction::Entity::find();

        for f in filter {
            query = query.filter(f);
        }

        query.count(self.db.as_ref()).await.unwrap_or_default()
    }

    pub async fn dump(
        &self,
        transaction: &entities::transaction::Model,
        _agony: bool,
    ) -> Option<TransactionResponse> {
        let mut response = TransactionResponse::builder();
        response
            .with_block(&transaction.chain_id)
            .with_hash(&transaction.tx_hash)
            .with_from_address(&transaction.from_address);

        if let Some(block_hash) = &transaction.block_hash {
            let block = self
                .block_manager
                .find(
                    &transaction.chain_id,
                    NumberOrHash::Hash(block_hash.clone()),
                )
                .await?;

            let block = self.block_manager.dump(&block, false).await?;

            response.with_block(block);
        }

        if let Some(tx_index) = transaction.tx_index {
            response.with_index(tx_index);
        }

        if let Some(to) = &transaction.to_address {
            response.with_to_address(to);
        }

        response.build().ok()
    }
}
