use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Block::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Block::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Block::ChainId).string().not_null())
                    .col(ColumnDef::new(Block::BlockNumber).big_integer().not_null())
                    .col(ColumnDef::new(Block::BlockHash).string().not_null())
                    .col(ColumnDef::new(Block::ParentHash).string().not_null())
                    .col(
                        ColumnDef::new(Block::TransactionCount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Block::ExtrinsicCount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Block::State).string().not_null())
                    .col(ColumnDef::new(Block::MinedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Block::FinalizedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Block::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Block::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_block_chain_id")
                    .table(Block::Table)
                    .col(Block::ChainId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_block_block_number")
                    .table(Block::Table)
                    .col(Block::BlockNumber)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_block_block_hash")
                    .table(Block::Table)
                    .col(Block::BlockHash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_block_parent_hash")
                    .table(Block::Table)
                    .col(Block::ParentHash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_block_state")
                    .table(Block::Table)
                    .col(Block::State)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Block::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Block {
    Table,
    Id,
    ChainId,
    BlockNumber,
    BlockHash,
    ParentHash,
    TransactionCount,
    ExtrinsicCount,
    State,
    MinedAt,
    FinalizedAt,
    CreatedAt,
    UpdatedAt,
}
