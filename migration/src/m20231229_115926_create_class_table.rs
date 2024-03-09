use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Class::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Class::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Class::Type).string().not_null())
                    .col(ColumnDef::new(Class::Name).string().not_null())
                    .col(ColumnDef::new(Class::Symbol).string().not_null())
                    .col(ColumnDef::new(Class::Owner).string())
                    .col(ColumnDef::new(Class::Description).string().not_null())
                    .col(ColumnDef::new(Class::CoverImageUri).string().not_null())
                    .col(ColumnDef::new(Class::ImageUriTemplate).string())
                    .col(ColumnDef::new(Class::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Class::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_class_owner")
                    .table(Class::Table)
                    .col(Class::Owner)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Class::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Class {
    Table,
    Id,
    Type,
    Name,
    Symbol,
    Owner,
    Description,
    CoverImageUri,
    ImageUriTemplate,
    CreatedAt,
    UpdatedAt,
}
