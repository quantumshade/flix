use std::path::Path;

use flix_tmdb::Client;

use anyhow::{Context, Result};
use clap::Parser;
use tokio::fs;

mod cli;
use cli::{BackendCommand, Cli, Command};

mod config;
use config::Config;
use tokio::io::AsyncWriteExt;

mod run;

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
		Command::Print { command } => match command {
			BackendCommand::Tmdb { command } => {
				let object = run::tmdb::TmdbObject::fetch(&client, command).await?;
				println!("{}", object.serialize().context("failed to serialize")?)
			}
		},
		Command::Write {
			force,
			output,
			command,
		} => match command {
			BackendCommand::Tmdb { command } => {
				let object = run::tmdb::TmdbObject::fetch(&client, command).await?;
				let output = output
					.as_ref()
					.map(|p| p.as_path())
					.unwrap_or(Path::new("flix.toml"));

				let mut file = if *force {
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
		},
	}

	Ok(())
}
