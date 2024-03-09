use std::sync::Arc;

use sea_orm::{
    sea_query::IntoCondition, ColumnTrait as _, DatabaseConnection, EntityTrait as _,
    IntoSimpleExpr, Order, QueryFilter as _, QueryOrder as _, QuerySelect as _,
};

use eos420_service_derive::cache;

use crate::{
    entities::{self, ContractType},
    managers::{ContractManager, LockedAssetManager},
    primitives::{v1::AssetResponse, Uint256},
    utilities::calculate_amount,
    CacheService, IdService,
};

#[di::injectable]
#[derive(Clone)]
pub struct AssetManager {
    pub cache: Arc<CacheService<entities::asset::Model>>,
    pub db: Arc<DatabaseConnection>,
    pub id: Arc<IdService>,
    pub contract_manager: Arc<ContractManager>,
    pub locked_asset_manager: Arc<LockedAssetManager>,
}

impl AssetManager {
    #[cache]
    pub async fn find(
        &self,
        chain_id: &str,
        asset_id: &str,
        address: &str,
    ) -> Option<entities::asset::Model> {
        let contract = self.contract_manager.find(chain_id, asset_id).await?;

        match contract.protocol {
            ContractType::Erc20 | ContractType::Eos20 => {}
            _ => return None,
        }

        entities::asset::Entity::find()
            .filter(entities::asset::Column::ContractId.eq(contract.id))
            .filter(entities::asset::Column::Address.eq(address))
            .order_by_desc(entities::asset::Column::Id)
            .one(self.db.as_ref())
            .await
            .ok()?
    }

    #[cache]
    pub async fn find_single(
        &self,
        chain_id: &str,
        asset_id: &str,
        identifier: &str,
    ) -> Option<entities::asset::Model> {
        let contract = self.contract_manager.find(chain_id, asset_id).await?;

        match contract.protocol {
            ContractType::Erc721 | ContractType::Eos420 => (),
            _ => return None,
        }

        entities::asset::Entity::find()
            .filter(entities::asset::Column::ContractId.eq(contract.id))
            .filter(entities::asset::Column::Value.eq(identifier))
            .order_by_desc(entities::asset::Column::Id)
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

    pub async fn dump(&self, asset: &entities::asset::Model, agony: bool) -> Option<AssetResponse> {
        let contract = self.contract_manager.get(asset.contract_id).await?;

        let mut response = AssetResponse::builder();

        let metadata = self.contract_manager.dump(&contract, false).await?;

        response.with_contract(metadata);

        if let Some(tx_hash) = asset.tx_hash.as_ref() {
            response.with_tx_hash(tx_hash);
        }

        if let Some(decimals) = contract.decimals {
            let amount = Uint256::from_str_prefixed(&asset.value).unwrap_or_default();
            let amount = calculate_amount(&amount, decimals);
            response.with_amount(amount);
        } else {
            response.with_identifier(&asset.value);
        }

        if agony {
            match contract.protocol {
                ContractType::Erc20 | ContractType::Eos20 => {
                    // TODO: Fungible
                }
                ContractType::Erc721 | ContractType::Eos420 => {
                    let locked = self
                        .locked_asset_manager
                        .single(&asset.chain_id, &asset.asset_id, &asset.value)
                        .await;
                    response.with_locked(locked.is_some());
                }
            }
        }

        response.build().ok()
    }
}
