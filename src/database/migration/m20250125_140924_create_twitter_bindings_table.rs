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
                    .col(ColumnDef::new(TwitterBinding::Uid).string().unique_key().not_null())
                    .col(ColumnDef::new(TwitterBinding::LamportId).string().not_null().unique_key())
                    .col(ColumnDef::new(TwitterBinding::XId).string().not_null().unique_key())
                    .col(ColumnDef::new(TwitterBinding::Name).string().not_null())
                    .col(ColumnDef::new(TwitterBinding::UserName).string().not_null().unique_key())
                    .col(ColumnDef::new(TwitterBinding::ImageUrl).string().not_null())
                    .col(ColumnDef::new(TwitterBinding::AccessToken).string().not_null())
                    .col(ColumnDef::new(TwitterBinding::RefreshToken).string().not_null())
                    .col(ColumnDef::new(TwitterBinding::TokenType).string().not_null())
                    .col(ColumnDef::new(TwitterBinding::Scope).string().not_null())
                    .col(ColumnDef::new(TwitterBinding::Retweet).integer().not_null().default(0))
                    .col(ColumnDef::new(TwitterBinding::Mention).integer().not_null().default(0))
                    .col(ColumnDef::new(TwitterBinding::Comment).integer().not_null().default(0))
                    .col(ColumnDef::new(TwitterBinding::Quote).integer().not_null().default(0))
                    .col(ColumnDef::new(TwitterBinding::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(TwitterBinding::CreatedAt).timestamp_with_time_zone().not_null())
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
    Uid,
    LamportId,
    XId,
    UserName,
    Name,
    ImageUrl,
    AccessToken,
    RefreshToken,
    TokenType,
    Scope,
    Retweet,
    Quote,
    Mention,
    Comment,
    UpdatedAt,
    CreatedAt,
}
