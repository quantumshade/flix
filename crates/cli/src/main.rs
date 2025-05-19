use std::path::Path;

use flix_tmdb::Client;

use anyhow::{Context, Result};
use clap::Parser;
use tokio::fs;
use tokio::io::AsyncWriteExt;

mod cli;
use cli::{BackendCommand, Cli, Command};

mod config;
use config::Config;

mod run;
use run::flix::FlixObject;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	let cli = Cli::parse();
	let config = fs::read_to_string(cli.config_path())
		.await
		.with_context(|| format!("could not read config: {:?}", cli.config_path()))?;
	let config: Config = toml::from_str(config.as_str())
		.with_context(|| format!("could not parse config: {:?}", cli.config_path()))?;

	let client = Client::new(config.tmdb().bearer_token().to_owned());

	match cli.command() {
		Command::Print { command } => exec_print(client, command).await?,
		Command::Write {
			force,
			output,
			command,
		} => exec_write(client, *force, output, command).await?,
		Command::Update { output } => exec_update(client, output).await?,
	}

	Ok(())
}

async fn exec_print(client: Client, command: &BackendCommand) -> Result<()> {
	match command {
		BackendCommand::Tmdb { command } => {
			let object = run::tmdb::TmdbObject::fetch(&client, command).await?;
			println!("{}", object.serialize().context("failed to serialize")?)
		}
	}
	Ok(())
}

async fn exec_write(
	client: Client,
	force: bool,
	output: &Path,
	command: &BackendCommand,
) -> Result<()> {
	match command {
		BackendCommand::Tmdb { command } => {
			let object = run::tmdb::TmdbObject::fetch(&client, command).await?;

			let mut file = if force {
				fs::File::create(output).await
			} else {
				fs::File::create_new(output).await
			}
			.with_context(|| format!("could not create file at path {}", output.display()))?;
			file.write_all(
				object
					.serialize()
					.context("failed to serialize tmdb object")?
					.as_bytes(),
			)
			.await
			.with_context(|| format!("could not write to file at path {}", output.display()))?;
		}
	}
	Ok(())
}

async fn exec_update(client: Client, output: &Path) -> Result<()> {
	let content = fs::read_to_string(output)
		.await
		.with_context(|| format!("failed to read file at path: {}", output.display()))?;
	let object: FlixObject = toml::from_str(&content).context("failed to deserialize flix file")?;

	let command = object.backend_command()?;
	exec_write(client, true, output, &command).await
}
