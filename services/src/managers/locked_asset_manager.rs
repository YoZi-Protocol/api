use std::sync::Arc;

use sea_orm::{
    sea_query::IntoCondition, ColumnTrait as _, DatabaseConnection, EntityTrait as _,
    IntoSimpleExpr, Order, QueryFilter as _, QueryOrder as _, QuerySelect as _,
};

use crate::{
    entities::{self, ContractType},
    managers::ContractManager,
    IdService,
};

#[di::injectable]
#[derive(Clone)]
pub struct LockedAssetManager {
    pub db: Arc<DatabaseConnection>,
    pub id: Arc<IdService>,
    pub contract_manager: Arc<ContractManager>,
}

impl LockedAssetManager {
    pub async fn single(
        &self,
        chain_id: &str,
        asset_id: &str,
        identifier: &str,
    ) -> Option<entities::locked_asset::Model> {
        let contract = self.contract_manager.find(chain_id, asset_id).await?;

        match contract.protocol {
            ContractType::Erc721 | ContractType::Eos420 => (),
            _ => return None,
        }

        entities::locked_asset::Entity::find()
            .filter(entities::locked_asset::Column::ContractId.eq(contract.id))
            .filter(entities::locked_asset::Column::Value.eq(identifier))
            .order_by_desc(entities::locked_asset::Column::Id)
            .one(self.db.as_ref())
            .await
            .ok()?
    }

    pub async fn query<C: IntoSimpleExpr, F: IntoCondition>(
        &self,
        filter: Vec<F>,
        order: Vec<(C, Order)>,
        limit: Option<u64>,
    ) -> Vec<entities::asset::Model> {
        let mut query = entities::asset::Entity::find();

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
}
