use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LamportId::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LamportId::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LamportId::CurrentValue).big_integer().not_null())
                    .col(
                        ColumnDef::new(LamportId::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LamportId::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Insert initial value
        manager
            .get_connection()
            .execute_unprepared(
                "INSERT INTO lamport_id (current_value, updated_at, created_at) VALUES (1, now(), now())",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LamportId::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum LamportId {
    Table,
    Id,
    CurrentValue,
    UpdatedAt,
    CreatedAt,
}

