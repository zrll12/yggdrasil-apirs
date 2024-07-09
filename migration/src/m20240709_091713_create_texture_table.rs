use sea_orm_migration::prelude::*;
use crate::m20240709_091713_create_texture_table::Texture::{Model, UploadTime};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Texture::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Texture::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Model).string_len(10).default("default".to_string()).not_null())
                    .col(ColumnDef::new(UploadTime).timestamp().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Texture::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Texture {
    Table,
    Id,
    Model,
    UploadTime,
}
