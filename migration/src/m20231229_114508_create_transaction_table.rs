use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transaction::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Transaction::ChainId).string().not_null())
                    .col(ColumnDef::new(Transaction::BlockNumber).big_integer())
                    .col(ColumnDef::new(Transaction::BlockHash).string())
                    .col(ColumnDef::new(Transaction::TxIndex).big_integer())
                    .col(ColumnDef::new(Transaction::TxHash).string().not_null())
                    .col(ColumnDef::new(Transaction::FromAddress).string().not_null())
                    .col(ColumnDef::new(Transaction::ToAddress).string())
                    .col(ColumnDef::new(Transaction::Value).string().not_null())
                    .col(ColumnDef::new(Transaction::ValueUsed).string().not_null())
                    .col(ColumnDef::new(Transaction::State).string().not_null())
                    .col(ColumnDef::new(Transaction::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Transaction::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_transaction_chain_id")
                    .table(Transaction::Table)
                    .col(Transaction::ChainId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_transaction_block_number")
                    .table(Transaction::Table)
                    .col(Transaction::BlockNumber)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_transaction_block_hash")
                    .table(Transaction::Table)
                    .col(Transaction::BlockHash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_transaction_tx_hash")
                    .table(Transaction::Table)
                    .col(Transaction::TxHash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_transaction_from_address")
                    .table(Transaction::Table)
                    .col(Transaction::FromAddress)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_transaction_to_address")
                    .table(Transaction::Table)
                    .col(Transaction::ToAddress)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_transaction_state")
                    .table(Transaction::Table)
                    .col(Transaction::State)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Transaction {
    Table,
    Id,
    ChainId,
    BlockNumber,
    BlockHash,
    TxIndex,
    TxHash,
    FromAddress,
    ToAddress,
    Value,
    ValueUsed,
    State,
    CreatedAt,
    UpdatedAt,
}
