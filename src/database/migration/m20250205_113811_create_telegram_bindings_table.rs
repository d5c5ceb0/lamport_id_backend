use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TelegramBinding::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TelegramBinding::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TelegramBinding::Uid).string().unique_key().not_null())
                    .col(ColumnDef::new(TelegramBinding::LamportId).string().not_null().unique_key())
                    .col(ColumnDef::new(TelegramBinding::TelegramId).string().not_null().unique_key())
                    .col(ColumnDef::new(TelegramBinding::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(TelegramBinding::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TelegramBinding::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TelegramBinding{
    Table,
    Id,
    Uid,
    LamportId,
    TelegramId,
    UpdatedAt,
    CreatedAt,
}
