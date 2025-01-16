use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VoteOption::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(VoteOption::Id).uuid().primary_key())
                    .col(ColumnDef::new(VoteOption::VoteId).uuid().not_null())
                    .col(ColumnDef::new(VoteOption::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(VoteOption::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VoteOption::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum VoteOption {
    Table,
    Id,
    VoteId,
    ProposalOptionId,
    CreatedAt,
    UpdatedAt,
}
