use std::sync::Arc;

use sea_orm::{
    ColumnTrait as _, DatabaseConnection, EntityTrait as _, PaginatorTrait, QueryFilter as _,
    QueryOrder as _, QuerySelect,
};

use eos420_service_derive::cache;

use crate::{
    entities::{self, ClassType},
    managers::ClassManager,
    primitives::{bigint::FromPrimitive as _, v1::ContractResponse, Uint256},
    CacheService, IdService,
};

#[di::injectable]
#[derive(Clone)]
pub struct ContractManager {
    pub cache: Arc<CacheService<entities::contract::Model>>,
    pub db: Arc<DatabaseConnection>,
    pub id: Arc<IdService>,
    pub class_manager: Arc<ClassManager>,
}

impl ContractManager {
    #[cache]
    pub async fn get(&self, id: i64) -> Option<entities::contract::Model> {
        entities::contract::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .ok()?
    }

    #[cache]
    pub async fn find(&self, chain_id: &str, asset_id: &str) -> Option<entities::contract::Model> {
        entities::contract::Entity::find()
            .filter(entities::contract::Column::ChainId.eq(chain_id))
            .filter(entities::contract::Column::AssetId.eq(asset_id))
            .order_by_desc(entities::contract::Column::Id)
            .one(self.db.as_ref())
            .await
            .ok()?
    }

    pub async fn supply(&self, contract: &entities::contract::Model) -> Option<Uint256> {
        match contract.protocol.into() {
            ClassType::Fungible => None,
            ClassType::NonFungible => {
                let count = entities::asset::Entity::find()
                    .filter(entities::asset::Column::ContractId.eq(contract.id))
                    .count(self.db.as_ref())
                    .await
                    .ok()?;

                Uint256::from_u64(count)
            }
        }
    }

    pub async fn holder(
        &self,
        contract: &entities::contract::Model,
        limit: u64,
    ) -> Vec<(String, i64)> {
        entities::asset::Entity::find()
            .select_only()
            .column(entities::asset::Column::Address)
            .column_as(
                entities::asset::Column::Id.count(),
                entities::asset::GroupAs::Count,
            )
            .filter(entities::asset::Column::ContractId.eq(contract.id))
            .group_by(entities::asset::Column::Address)
            .order_by_desc(entities::asset::GroupAs::Count)
            .limit(limit)
            .into_values::<_, entities::asset::GroupAs>()
            .all(self.db.as_ref())
            .await
            .unwrap_or_default()
    }

    pub async fn holder_count(&self, contract: &entities::contract::Model) -> u64 {
        entities::asset::Entity::find()
            .select_only()
            .column(entities::asset::Column::Address)
            .column_as(
                entities::asset::Column::Id.count(),
                entities::asset::GroupAs::Count,
            )
            .filter(entities::asset::Column::ContractId.eq(contract.id))
            .group_by(entities::asset::Column::Address)
            .count(self.db.as_ref())
            .await
            .unwrap_or_default()
    }

    pub async fn dump(
        &self,
        contract: &entities::contract::Model,
        agony: bool,
    ) -> Option<ContractResponse> {
        let class = self.class_manager.get(contract.class_id).await?;

        let mut response = ContractResponse::builder();
        response
            .with_chain_id(&contract.chain_id)
            .with_id(&contract.asset_id)
            .with_type(class.r#type)
            .with_protocol(contract.protocol)
            .with_name(&class.name)
            .with_symbol(&class.symbol)
            .with_description(&class.description)
            .with_cover_image_uri(&class.cover_image_uri)
            .with_state(contract.state);

        response
            .with_to_address(&contract.address)
            .with_fee(Uint256::from_str_prefixed("0x1A055690D9DB80000").unwrap());

        if let Some(owner) = &contract.owner {
            response.with_owner(owner);
        }

        if let Some(decimals) = contract.decimals {
            response.with_decimals(decimals);
        }

        if let Some(max_supply) = &contract.max_supply {
            let max_supply = Uint256::from_str_prefixed(max_supply).unwrap_or_default();

            response.with_max_supply(max_supply);
        }

        if let Some(mint_limit) = &contract.mint_limit {
            let mint_limit = Uint256::from_str_prefixed(mint_limit).unwrap_or_default();

            response.with_mint_limit(mint_limit);
        }

        if let Some(not_before) = contract.not_before {
            response.with_not_before(not_before);
        }

        if let Some(deployed_at) = contract.deployed_at {
            response.with_deployed_at(deployed_at);
        }

        if let Some(tx_hash) = &contract.tx_hash {
            response.with_tx_hash(tx_hash);
        }

        if agony {
            response
                .with_supply(self.supply(contract).await.unwrap_or_default())
                .with_holder_count(self.holder_count(contract).await);
        }

        response.build().ok()
    }
}
