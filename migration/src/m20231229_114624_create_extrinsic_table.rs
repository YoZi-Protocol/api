use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Extrinsic::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Extrinsic::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Extrinsic::ChainId).string().not_null())
                    .col(ColumnDef::new(Extrinsic::BlockNumber).big_integer())
                    .col(ColumnDef::new(Extrinsic::BlockHash).string())
                    .col(ColumnDef::new(Extrinsic::TxIndex).big_integer())
                    .col(ColumnDef::new(Extrinsic::TxHash).string().not_null())
                    .col(ColumnDef::new(Extrinsic::Index).big_integer().not_null())
                    .col(ColumnDef::new(Extrinsic::AssetId).string().not_null())
                    .col(ColumnDef::new(Extrinsic::Protocol).string().not_null())
                    .col(ColumnDef::new(Extrinsic::FromAddress).string().not_null())
                    .col(ColumnDef::new(Extrinsic::ToAddress).string().not_null())
                    .col(ColumnDef::new(Extrinsic::Operation).string().not_null())
                    .col(ColumnDef::new(Extrinsic::Value).string().not_null())
                    .col(ColumnDef::new(Extrinsic::Context).string())
                    .col(ColumnDef::new(Extrinsic::State).string().not_null())
                    .col(ColumnDef::new(Extrinsic::DropReason).string())
                    .col(ColumnDef::new(Extrinsic::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Extrinsic::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_extrinsic_chain_id")
                    .table(Extrinsic::Table)
                    .col(Extrinsic::ChainId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_extrinsic_block_number")
                    .table(Extrinsic::Table)
                    .col(Extrinsic::BlockNumber)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_extrinsic_block_hash")
                    .table(Extrinsic::Table)
                    .col(Extrinsic::BlockHash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_extrinsic_tx_hash")
                    .table(Extrinsic::Table)
                    .col(Extrinsic::TxHash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_extrinsic_asset_id")
                    .table(Extrinsic::Table)
                    .col(Extrinsic::AssetId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_extrinsic_protocol")
                    .table(Extrinsic::Table)
                    .col(Extrinsic::Protocol)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_extrinsic_from_address")
                    .table(Extrinsic::Table)
                    .col(Extrinsic::FromAddress)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_extrinsic_to_address")
                    .table(Extrinsic::Table)
                    .col(Extrinsic::ToAddress)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_extrinsic_operation")
                    .table(Extrinsic::Table)
                    .col(Extrinsic::Operation)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_extrinsic_state")
                    .table(Extrinsic::Table)
                    .col(Extrinsic::State)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Extrinsic::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Extrinsic {
    Table,
    Id,
    ChainId,
    BlockNumber,
    BlockHash,
    TxIndex,
    TxHash,
    Index,
    AssetId,
    Protocol,
    FromAddress,
    ToAddress,
    Operation,
    Value,
    Context,
    State,
    DropReason,
    CreatedAt,
    UpdatedAt,
}
