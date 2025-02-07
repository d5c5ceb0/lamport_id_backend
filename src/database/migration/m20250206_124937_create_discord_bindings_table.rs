use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DiscordBinding::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DiscordBinding::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DiscordBinding::Uid).string().unique_key().not_null())
                    .col(ColumnDef::new(DiscordBinding::LamportId).string().not_null().unique_key())
                    .col(ColumnDef::new(DiscordBinding::DiscordId).string().not_null().unique_key())
                    .col(ColumnDef::new(DiscordBinding::UserName).string().not_null())
                    .col(ColumnDef::new(DiscordBinding::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(DiscordBinding::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DiscordBinding::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum DiscordBinding {
    Table,
    Id,
    Uid,
    LamportId,
    DiscordId,
    UserName,
    UpdatedAt,
    CreatedAt,
}
