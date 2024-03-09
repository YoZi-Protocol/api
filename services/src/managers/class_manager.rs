use std::sync::Arc;

use sea_orm::{DatabaseConnection, EntityTrait as _};

use eos420_service_derive::cache;

use crate::{entities, CacheService, IdService};

#[di::injectable]
#[derive(Clone)]
pub struct ClassManager {
    pub cache: Arc<CacheService<entities::class::Model>>,
    pub db: Arc<DatabaseConnection>,
    pub id: Arc<IdService>,
}

impl ClassManager {
    #[cache]
    pub async fn get(&self, id: i64) -> Option<entities::class::Model> {
        entities::class::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .ok()?
    }
}
