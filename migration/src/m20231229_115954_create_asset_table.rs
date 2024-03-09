use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Asset::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Asset::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Asset::ClassId).big_integer().not_null())
                    .col(ColumnDef::new(Asset::ContractId).big_integer().not_null())
                    .col(ColumnDef::new(Asset::ChainId).string().not_null())
                    .col(ColumnDef::new(Asset::AssetId).string().not_null())
                    .col(ColumnDef::new(Asset::TxHash).string())
                    .col(ColumnDef::new(Asset::Index).big_integer())
                    .col(ColumnDef::new(Asset::Address).string().not_null())
                    .col(ColumnDef::new(Asset::Value).string().not_null())
                    .col(ColumnDef::new(Asset::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Asset::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_asset_class_id")
                    .table(Asset::Table)
                    .col(Asset::ClassId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_asset_contract_id")
                    .table(Asset::Table)
                    .col(Asset::ContractId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_asset_chain_id")
                    .table(Asset::Table)
                    .col(Asset::ChainId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_asset_asset_id")
                    .table(Asset::Table)
                    .col(Asset::AssetId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_asset_tx_hash")
                    .table(Asset::Table)
                    .col(Asset::TxHash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_asset_address")
                    .table(Asset::Table)
                    .col(Asset::Address)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Asset::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Asset {
    Table,
    Id,
    ClassId,
    ContractId,
    ChainId,
    AssetId,
    TxHash,
    Index,
    Address,
    Value,
    CreatedAt,
    UpdatedAt,
}
