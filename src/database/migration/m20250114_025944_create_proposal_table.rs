use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                .table(Proposal::Table)
                .col(ColumnDef::new(Proposal::Id).integer().primary_key().auto_increment())
                .col(ColumnDef::new(Proposal::ProposerId).string().not_null())
                .col(ColumnDef::new(Proposal::Title).string().not_null())
                .col(ColumnDef::new(Proposal::Description).text().not_null())
                .col(ColumnDef::new(Proposal::Options).array(ColumnType::String(StringLen::N(127))).not_null())
                .col(ColumnDef::new(Proposal::CreatedBy).string().not_null())
                .col(ColumnDef::new(Proposal::StartTime).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Proposal::EndTime).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Proposal::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Proposal::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Proposal::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Proposal {
    Table,
    Id,
    ProposerId,
    CreatedBy,
    Options,
    Title,
    Description,
    StartTime,
    EndTime,
    CreatedAt,
    UpdatedAt,
}


