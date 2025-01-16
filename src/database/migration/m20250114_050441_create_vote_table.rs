use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                .table(Vote::Table)
                .col(ColumnDef::new(Vote::Id).integer().primary_key().auto_increment())
                .col(ColumnDef::new(Vote::ProposalId).uuid().not_null())
                .col(ColumnDef::new(Vote::VoterId).uuid().not_null())
                .col(ColumnDef::new(Vote::CreatedAt).timestamp().not_null())
                .col(ColumnDef::new(Vote::UpdatedAt).timestamp().not_null())
                .to_owned(),
        )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vote::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
enum Vote {
    Table,
    Id,
    ProposalId,
    VoterId,
    CreatedAt,
    UpdatedAt,
}

