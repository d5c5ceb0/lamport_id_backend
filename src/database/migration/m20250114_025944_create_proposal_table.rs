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
                .col(ColumnDef::new(Proposal::Title).string().not_null())
                .col(ColumnDef::new(Proposal::Description).text())
                .col(ColumnDef::new(Proposal::ProposerId).uuid().not_null())
                .col(ColumnDef::new(Proposal::IsMultiOption).boolean().not_null())
                .col(ColumnDef::new(Proposal::StartTime).timestamp().not_null())
                .col(ColumnDef::new(Proposal::EndTime).timestamp().not_null())
                .col(ColumnDef::new(Proposal::Status).string().not_null())
                .col(ColumnDef::new(Proposal::CreatedAt).timestamp().not_null())
                .col(ColumnDef::new(Proposal::UpdatedAt).timestamp().not_null())
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
    Title,
    Description,
    IsMultiOption,
    StartTime,
    EndTime,
    Status,
    CreatedAt,
    UpdatedAt,
}

