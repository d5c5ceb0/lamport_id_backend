use crate::database::migration::Migrator;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use cli::CommandHandler;
use db::setup_db;
use crate::common::config::Config;
use sea_orm::*;

pub struct MigrateCommand;

#[async_trait]
impl CommandHandler for MigrateCommand {
    fn name(&self) -> String {
        "migrate".to_string()
    }

    fn define(&self) -> Command {
        Command::new("migrate").about("migrate database").arg(
            Arg::new("db_url")
                .short('d')
                .long("db_url")
                .value_parser(clap::value_parser!(String))
                .help("database url"),
        ).arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_parser(clap::value_parser!(String))
                .help("config file path"),
        )
    }

    async fn run(&self, matches: &ArgMatches) {
        if let Some(db_url) = matches.get_one::<String>("db_url") {
            if let Ok(url) = url::Url::parse(db_url) {
                let db_name = url.path().trim_start_matches('/');
                let base_url = url.as_str().trim_end_matches(db_name);
                setup_db::<Migrator>(base_url, db_name).await.unwrap();
            }
        }

        if let Some(config_file) = matches.get_one::<String>("config") {
            let config = Config::load_config(config_file.into()).unwrap();
            if let Ok(url) = url::Url::parse(&config.database.db_url) {
                let db_name = url.path().trim_start_matches('/');
                let base_url = url.as_str().trim_end_matches(db_name);
                let rdb = setup_db::<Migrator>(base_url, db_name).await.unwrap();

                rdb.execute(Statement::from_string(
                        rdb.get_database_backend(),
                        "INSERT INTO groups (group_id, name, logo, description, website, twitter, created_by, created_at, updated_at) VALUES ('293dbe4f-0b6b-462d-a778-2dceab12256b', 'AI4Sci DAO', 'https://github.com/d5c5ceb0/t/blob/main/images.png', 'AI4Sci DAO is a DAO focused on AI governance', 'website1', 'twitter1', 'created_by1', '2025-01-01 00:00:00', '2025-01-01 00:00:00');".to_string(),
                )).await.unwrap();

            }
        }

    }
}
