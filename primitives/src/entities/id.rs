use crate::entities;

pub trait EntityId {
    fn id(&self) -> i64;
}

impl EntityId for entities::asset::Model {
    fn id(&self) -> i64 {
        self.id
    }
}

impl EntityId for entities::class::Model {
    fn id(&self) -> i64 {
        self.id
    }
}

impl EntityId for entities::contract::Model {
    fn id(&self) -> i64 {
        self.id
    }
}

impl EntityId for entities::extrinsic::Model {
    fn id(&self) -> i64 {
        self.id
    }
}

impl EntityId for entities::transaction::Model {
    fn id(&self) -> i64 {
        self.id
    }
}
