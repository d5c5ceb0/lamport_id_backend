use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GithubBinding::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GithubBinding::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GithubBinding::Uid).string().unique_key().not_null())
                    .col(ColumnDef::new(GithubBinding::LamportId).string().not_null().unique_key())
                    .col(ColumnDef::new(GithubBinding::GithubId).string().not_null().unique_key())
                    .col(ColumnDef::new(GithubBinding::UserName).string().not_null())
                    .col(ColumnDef::new(GithubBinding::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(GithubBinding::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GithubBinding::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GithubBinding{
    Table,
    Id,
    Uid,
    LamportId,
    GithubId,
    UserName,
    UpdatedAt,
    CreatedAt,
}
