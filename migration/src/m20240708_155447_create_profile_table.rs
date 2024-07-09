use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Profile::Id)
                            .string_len(32)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Profile::Name).string().not_null())
                    .col(ColumnDef::new(Profile::Model).string_len(10).default("default".to_string()).not_null())
                    .col(ColumnDef::new(Profile::OwnerId).string().not_null())
                    .col(ColumnDef::new(Profile::SkinTexture).string().null())
                    .col(ColumnDef::new(Profile::CapeTexture).string().null())
                    .col(ColumnDef::new(Profile::CreateTime).timestamp().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Profile::UpdateTime).timestamp().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Profile {
    Table,
    Id,
    Name,
    Model,
    OwnerId,
    SkinTexture,
    CapeTexture,
    CreateTime,
    UpdateTime,
}
