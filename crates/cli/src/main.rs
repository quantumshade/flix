use std::path::PathBuf;

use flix::tmdb::Client;

use anyhow::{Context, Result};
use clap::Parser;
use tokio::fs;

mod cli;
use cli::{AddCommand, Cli, Command, DeleteCommand, UpdateCommand};

mod config;
use config::Config;

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

	let client = Client::new(config.tmdb().bearer_token().to_owned());

	if cli.trace {
		tracing_subscriber::fmt()
			.with_max_level(tracing::Level::DEBUG)
			.with_test_writer()
			.init();
	}

	match cli.command() {
		Command::Init => exec_init(database_path).await?,
		Command::Add { command } => exec_add(client, database_path, command).await?,
		Command::Update { command } => exec_update(client, database_path, command).await?,
		Command::Delete { command } => exec_delete(client, database_path, command).await?,
		Command::Backup { output } => exec_backup(database_path, output).await?,
		Command::Restore { input } => exec_restore(database_path, input).await?,
	}

	Ok(())
}

async fn exec_init(database_path: String) -> Result<()> {
	db::open_new(database_path).await?;

	Ok(())
}

async fn exec_add(client: Client, database_path: String, command: AddCommand) -> Result<()> {
	let database = db::open(database_path).await?;

	match command {
		AddCommand::Flix { command } => {
			run::flix::add(database.as_ref(), command).await?;
		}
		AddCommand::Tmdb { command } => {
			run::tmdb::add(client, database.as_ref(), command).await?;
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

async fn exec_delete(client: Client, database_path: String, command: DeleteCommand) -> Result<()> {
	_ = client;
	_ = database_path;
	_ = command;
	unimplemented!()
}

async fn exec_backup(database_path: String, output: PathBuf) -> Result<()> {
	_ = database_path;
	_ = output;
	unimplemented!()
}

async fn exec_restore(database_path: String, input: PathBuf) -> Result<()> {
	_ = database_path;
	_ = input;
	unimplemented!()
}
