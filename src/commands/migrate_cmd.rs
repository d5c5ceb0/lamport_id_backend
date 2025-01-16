use crate::database::migration::Migrator;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use cli::CommandHandler;
use db::setup_db;
use crate::common::config::Config;

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
            if let Ok(url) = url::Url::parse(&db_url) {
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
                setup_db::<Migrator>(base_url, db_name).await.unwrap();
            }
        }

    }
}
