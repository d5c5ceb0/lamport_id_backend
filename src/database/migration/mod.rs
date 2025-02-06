pub use sea_orm_migration::prelude::*;

mod m20241223_064204_create_users_table;
mod m20241225_025628_create_points_table;
mod m20250114_025944_create_proposal_table;
mod m20250114_050441_create_vote_table;
mod m20250114_052617_create_power_table;
mod m20250116_063026_create_lamport_id_table;
mod m20250116_131947_create_group_table;
mod m20250125_140924_create_twitter_bindings_table;
mod m20250128_081521_create_events_table;
mod m20250205_113811_create_telegram_bindings_table;
mod m20250206_124937_create_discord_bindings_table;
mod m20250206_142515_create_github_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241223_064204_create_users_table::Migration),
            Box::new(m20241225_025628_create_points_table::Migration),
            Box::new(m20250114_025944_create_proposal_table::Migration),
            Box::new(m20250114_050441_create_vote_table::Migration),
            Box::new(m20250114_052617_create_power_table::Migration),
            Box::new(m20250116_063026_create_lamport_id_table::Migration),
            Box::new(m20250116_131947_create_group_table::Migration),
            Box::new(m20250125_140924_create_twitter_bindings_table::Migration),
            Box::new(m20250128_081521_create_events_table::Migration),
            Box::new(m20250205_113811_create_telegram_bindings_table::Migration),
            Box::new(m20250206_124937_create_discord_bindings_table::Migration),
            Box::new(m20250206_142515_create_github_table::Migration),
        ]
    }
}
