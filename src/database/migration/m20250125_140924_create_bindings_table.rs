use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TwitterBinding::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TwitterBinding::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TwitterBinding::UserId).string().not_null())
                    .col(ColumnDef::new(TwitterBinding::XId).string().not_null().unique_key())
                    .col(ColumnDef::new(TwitterBinding::Name).string().not_null())
                    .col(ColumnDef::new(TwitterBinding::UserName).string().not_null().unique_key())
                    .col(ColumnDef::new(TwitterBinding::ImageUrl).string().not_null())
                    .col(ColumnDef::new(TwitterBinding::AccessToken).string().not_null())
                    .col(ColumnDef::new(TwitterBinding::RefreshToken).string().not_null())
                    .col(ColumnDef::new(TwitterBinding::TokenType).string().not_null())
                    .col(ColumnDef::new(TwitterBinding::Scope).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TwitterBinding::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TwitterBinding {
    Table,
    Id,
    UserId,  //lamport_id
    XId,
    UserName,
    Name,
    ImageUrl,
    AccessToken,
    RefreshToken,
    TokenType,
    Scope,
}
