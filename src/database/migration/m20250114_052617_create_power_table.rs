use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Power::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Power::Id).integer().primary_key().auto_increment())
                    .col(ColumnDef::new(Power::LamportId).string().not_null())
                    .col(ColumnDef::new(Power::Amounts).integer().not_null().default(0))
                    .col(ColumnDef::new(Power::Types).string().not_null())
                    .col(ColumnDef::new(Power::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Power::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Power {
    Table,
    Id,
    LamportId,
    Types,
    Amounts,
    CreatedAt,
}

