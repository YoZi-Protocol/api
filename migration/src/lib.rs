pub use sea_orm_migration::prelude::*;

mod m20231229_114508_create_transaction_table;
mod m20231229_114624_create_extrinsic_table;
mod m20231229_115926_create_class_table;
mod m20231229_115937_create_contract_table;
mod m20231229_115954_create_asset_table;
mod m20240106_091416_create_block_table;
mod m20240119_143933_create_locked_asset_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231229_114508_create_transaction_table::Migration),
            Box::new(m20231229_114624_create_extrinsic_table::Migration),
            Box::new(m20231229_115926_create_class_table::Migration),
            Box::new(m20231229_115937_create_contract_table::Migration),
            Box::new(m20231229_115954_create_asset_table::Migration),
            Box::new(m20240106_091416_create_block_table::Migration),
            Box::new(m20240119_143933_create_locked_asset_table::Migration),
        ]
    }
}
