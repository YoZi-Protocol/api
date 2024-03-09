use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LockedAsset::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LockedAsset::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LockedAsset::ClassId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LockedAsset::ContractId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(LockedAsset::ChainId).string().not_null())
                    .col(ColumnDef::new(LockedAsset::AssetId).string().not_null())
                    .col(ColumnDef::new(LockedAsset::Address).string().not_null())
                    .col(ColumnDef::new(LockedAsset::Delegate).string().not_null())
                    .col(ColumnDef::new(LockedAsset::Nonce).string().not_null())
                    .col(ColumnDef::new(LockedAsset::Value).string().not_null())
                    .col(ColumnDef::new(LockedAsset::LockReason).string().not_null())
                    .col(
                        ColumnDef::new(LockedAsset::ExpiresAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(LockedAsset::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(LockedAsset::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_locked_asset_class_id")
                    .table(LockedAsset::Table)
                    .col(LockedAsset::ClassId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_locked_asset_contract_id")
                    .table(LockedAsset::Table)
                    .col(LockedAsset::ContractId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_locked_asset_chain_id")
                    .table(LockedAsset::Table)
                    .col(LockedAsset::ChainId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_locked_asset_asset_id")
                    .table(LockedAsset::Table)
                    .col(LockedAsset::AssetId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_locked_asset_address")
                    .table(LockedAsset::Table)
                    .col(LockedAsset::Address)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_locked_asset_delegate")
                    .table(LockedAsset::Table)
                    .col(LockedAsset::Delegate)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LockedAsset::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LockedAsset {
    Table,
    Id,
    ClassId,
    ContractId,
    ChainId,
    AssetId,
    Address,
    Delegate,
    Nonce,
    Value,
    LockReason,
    ExpiresAt,
    CreatedAt,
    UpdatedAt,
}
