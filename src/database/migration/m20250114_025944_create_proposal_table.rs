use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                .table(Proposals::Table)
                .col(ColumnDef::new(Proposals::Id).integer().primary_key().auto_increment())
                .col(ColumnDef::new(Proposals::ProposalId).string().not_null())
                .col(ColumnDef::new(Proposals::GroupId).string().not_null())
                .col(ColumnDef::new(Proposals::Title).string().not_null())
                .col(ColumnDef::new(Proposals::Description).text().not_null())
                .col(ColumnDef::new(Proposals::Options).array(ColumnType::String(StringLen::N(127))).not_null())
                .col(ColumnDef::new(Proposals::CreatedBy).string().not_null())
                .col(ColumnDef::new(Proposals::StartTime).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Proposals::EndTime).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Proposals::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Proposals::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Proposals::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Proposals {
    Table,
    Id,
    ProposalId,
    GroupId,
    CreatedBy,
    Options,
    Title,
    Description,
    StartTime,
    EndTime,
    CreatedAt,
    UpdatedAt,
}


