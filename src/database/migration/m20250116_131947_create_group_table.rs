use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Groups::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Groups::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Groups::GroupId).string().unique_key().not_null())
                    .col(ColumnDef::new(Groups::Name).string().not_null())
                    .col(ColumnDef::new(Groups::Logo).string().not_null())
                    .col(ColumnDef::new(Groups::Description).text().null())
                    .col(ColumnDef::new(Groups::Website).string().not_null())
                    .col(ColumnDef::new(Groups::Twitter).string().not_null())
                    .col(ColumnDef::new(Groups::CreatedBy).string().not_null())
                    .col(ColumnDef::new(Groups::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Groups::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Groups::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Groups {
    Table,
    Id,
    GroupId,
    Name,
    Logo,
    Description,
    Website,
    Twitter,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
