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
                .col(ColumnDef::new(Vote::Uid).string().not_null().unique_key())
                .col(ColumnDef::new(Vote::VoterId).string().not_null())
                .col(ColumnDef::new(Vote::ProposalId).string().not_null())
                .col(ColumnDef::new(Vote::Choice).string().not_null())
                .col(ColumnDef::new(Vote::Channel).string().not_null())
                .col(ColumnDef::new(Vote::CreatedAt).timestamp_with_time_zone().not_null())
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
    Uid,
    VoterId,
    ProposalId,
    Choice,
    Channel,
    CreatedAt,
}

