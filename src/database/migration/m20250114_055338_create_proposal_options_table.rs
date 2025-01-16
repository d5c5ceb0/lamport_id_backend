use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProposalOption::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ProposalOption::Id).uuid().primary_key())
                    .col(ColumnDef::new(ProposalOption::ProposalId).string().not_null())
                    .col(ColumnDef::new(ProposalOption::Content).string().not_null())
                    .col(ColumnDef::new(ProposalOption::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ProposalOption::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProposalOption::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum ProposalOption {
    Table,
    Id,
    ProposalId,
    Content,
    CreatedAt,
    UpdatedAt,
}

