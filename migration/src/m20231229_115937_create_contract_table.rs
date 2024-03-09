use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Contract::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Contract::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Contract::ClassId).big_integer().not_null())
                    .col(ColumnDef::new(Contract::ChainId).string().not_null())
                    .col(ColumnDef::new(Contract::AssetId).string().not_null())
                    .col(ColumnDef::new(Contract::Address).string().not_null())
                    .col(ColumnDef::new(Contract::Owner).string())
                    .col(ColumnDef::new(Contract::Protocol).string().not_null())
                    .col(ColumnDef::new(Contract::Decimals).integer())
                    .col(ColumnDef::new(Contract::Identifier).string())
                    .col(ColumnDef::new(Contract::MaxSupply).string())
                    .col(ColumnDef::new(Contract::MintLimit).string())
                    .col(ColumnDef::new(Contract::NotBefore).big_integer())
                    .col(ColumnDef::new(Contract::TxHash).string())
                    .col(ColumnDef::new(Contract::State).string().not_null())
                    .col(ColumnDef::new(Contract::DeployedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Contract::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Contract::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_contract_class_id")
                    .table(Contract::Table)
                    .col(Contract::ClassId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_contract_chain_id")
                    .table(Contract::Table)
                    .col(Contract::ChainId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_contract_asset_id")
                    .table(Contract::Table)
                    .col(Contract::AssetId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_contract_address")
                    .table(Contract::Table)
                    .col(Contract::Address)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_contract_owner")
                    .table(Contract::Table)
                    .col(Contract::Owner)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_contract_protocol")
                    .table(Contract::Table)
                    .col(Contract::Protocol)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_contract_tx_hash")
                    .table(Contract::Table)
                    .col(Contract::TxHash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_contract_state")
                    .table(Contract::Table)
                    .col(Contract::State)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contract::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Contract {
    Table,
    Id,
    ClassId,
    ChainId,
    AssetId,
    Address,
    Owner,
    Protocol,
    Decimals,
    Identifier,
    MaxSupply,
    MintLimit,
    NotBefore,
    TxHash,
    State,
    DeployedAt,
    CreatedAt,
    UpdatedAt,
}
