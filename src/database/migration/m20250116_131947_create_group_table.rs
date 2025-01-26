use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Group::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Group::GroupId).string().unique_key().not_null())
                    .col(ColumnDef::new(Group::Name).string().not_null())
                    .col(ColumnDef::new(Group::Logo).string().not_null())
                    .col(ColumnDef::new(Group::Description).text().null())
                    .col(ColumnDef::new(Group::Website).string().not_null())
                    .col(ColumnDef::new(Group::Twitter).string().not_null())
                    .col(ColumnDef::new(Group::CreatedBy).string().not_null())
                    .col(ColumnDef::new(Group::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Group::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Group {
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
