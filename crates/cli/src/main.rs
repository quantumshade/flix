//! flix-cli.

extern crate alloc;

use std::path::Path;

use alloc::sync::Arc;

use flix::tmdb::{self, CachePolicy, Client, RedbCache};

use anyhow::{Context as _, Result};
use clap::Parser as _;
use tokio::fs;

mod cli;
use cli::{AddCommand, Cli, Command};

mod config;
use config::Config;

use crate::cli::AddOverrides;

mod db;
mod run;

#[tokio::main(flavor = "current_thread")]
#[cfg_attr(test, expect(clippy::missing_errors_doc, reason = "main function"))]
#[cfg_attr(test, expect(clippy::missing_panics_doc, reason = "main function "))]
async fn main() -> Result<()> {
	let cli = Cli::parse();
	let config = fs::read_to_string(cli.config_path())
		.await
		.with_context(|| format!("could not read config: {}", cli.config_path().display()))?;
	let config: Config = toml::from_str(config.as_str())
		.with_context(|| format!("could not parse config: {}", cli.config_path().display()))?;

	let database_path = cli.database_path()?;
	let database_path = Path::new(&database_path);

	let config = tmdb::Config::new(config.tmdb().bearer_token().to_owned());
	let cache = Arc::new(RedbCache::new(cli.cache_path())?);
	let client = Client::new(config, cache, CachePolicy::Full);

	if cli.trace() {
		tracing_subscriber::fmt()
			.with_max_level(tracing::Level::DEBUG)
			.with_test_writer()
			.init();
	}

	match cli.get_command() {
		Command::Init => exec_init(database_path).await?,
		Command::Add { command, overrides } => {
			exec_add(client, database_path, command, overrides).await?;
		}
	}

	Ok(())
}

/// Execute the `init` command.
///
/// # Errors
/// Forwards any errors.
async fn exec_init(database_path: &Path) -> Result<()> {
	drop(db::open_new(database_path).await?);

	Ok(())
}

/// Execute the `add` command.
///
/// # Errors
/// Forwards any errors.
async fn exec_add(
	client: Client,
	database_path: &Path,
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
