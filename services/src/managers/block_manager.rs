use std::sync::Arc;

use sea_orm::{
    sea_query::IntoCondition, ColumnTrait as _, DatabaseConnection, EntityTrait as _,
    IntoSimpleExpr, Order, QueryFilter as _, QueryOrder as _, QuerySelect as _,
};

use eos420_service_derive::cache;

use crate::{
    entities::{self, NumberOrHash},
    primitives::v1::BlockResponse,
    CacheService, IdService,
};

#[di::injectable]
#[derive(Clone)]
pub struct BlockManager {
    pub cache: Arc<CacheService<entities::block::Model>>,
    pub db: Arc<DatabaseConnection>,
    pub id: Arc<IdService>,
}

impl BlockManager {
    #[cache]
    pub async fn find(
        &self,
        chain_id: &str,
        block: NumberOrHash,
    ) -> Option<entities::block::Model> {
        let filter = match block {
            NumberOrHash::Number(block_number) => {
                entities::block::Column::BlockNumber.eq(block_number)
            }
            NumberOrHash::Hash(block_hash) => entities::block::Column::BlockHash.eq(block_hash),
        };

        entities::block::Entity::find()
            .filter(entities::block::Column::ChainId.eq(chain_id))
            .filter(filter)
            .order_by_desc(entities::block::Column::Id)
            .one(self.db.as_ref())
            .await
            .ok()?
    }

    pub async fn query<C: IntoSimpleExpr, F: IntoCondition>(
        &self,
        filter: Vec<F>,
        order: Vec<(C, Order)>,
        limit: Option<u64>,
    ) -> Vec<entities::block::Model> {
        let mut query = entities::block::Entity::find();

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
        block: &entities::block::Model,
        _agony: bool,
    ) -> Option<BlockResponse> {
        let mut response = BlockResponse::builder();
        response
            .with_chain_id(&block.chain_id)
            .with_number(block.block_number)
            .with_hash(&block.block_hash)
            .with_state(block.state);

        if let Some(finalized_at) = block.finalized_at {
            response.with_finalized(true);
            response.with_finalized_at(finalized_at);
        }

        response.build().ok()
    }
}
