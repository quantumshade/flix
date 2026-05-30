//! flix-cli

use std::rc::Rc;

use flix::tmdb::{self, CachePolicy, Client, RedbCache};

use anyhow::{Context, Result};
use clap::Parser;
use tokio::fs;

mod cli;
use cli::{AddCommand, Cli, Command, UpdateCommand};

mod config;
use config::Config;

use crate::cli::AddOverrides;

mod db;
mod run;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	let cli = Cli::parse();
	let config = fs::read_to_string(cli.config_path())
		.await
		.with_context(|| format!("could not read config: {:?}", cli.config_path()))?;
	let config: Config = toml::from_str(config.as_str())
		.with_context(|| format!("could not parse config: {:?}", cli.config_path()))?;

	let database_path = cli.database_path()?;

	let config = tmdb::Config::new(config.tmdb().bearer_token().to_owned());
	let cache = Rc::new(RedbCache::new(cli.cache_path())?);
	let client = Client::new(config, cache, CachePolicy::Full);

	if cli.trace {
		tracing_subscriber::fmt()
			.with_max_level(tracing::Level::DEBUG)
			.with_test_writer()
			.init();
	}

	match cli.command() {
		Command::Init => exec_init(database_path).await?,
		Command::Add { command, overrides } => {
			exec_add(client, database_path, command, overrides).await?
		}
		Command::Update { command } => exec_update(client, database_path, command).await?,
	}

	Ok(())
}

async fn exec_init(database_path: String) -> Result<()> {
	db::open_new(database_path).await?;

	Ok(())
}

async fn exec_add(
	client: Client,
	database_path: String,
	command: AddCommand,
	overrides: AddOverrides,
) -> Result<()> {
	let database = db::open(database_path).await?;

	match command {
		AddCommand::Flix { command } => {
			run::flix::add(database.as_ref(), command, overrides).await?;
		}
		AddCommand::Tmdb { command } => {
			run::tmdb::add(client, database.as_ref(), command, overrides).await?;
		}
	}

	Ok(())
}

async fn exec_update(client: Client, database_path: String, command: UpdateCommand) -> Result<()> {
	let database = db::open(database_path).await?;

	match command {
		UpdateCommand::Tmdb { command } => {
			run::tmdb::update(client, database.as_ref(), command).await?;
		}
	}

	Ok(())
}
