use std::path::PathBuf;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};

pub mod tmdb;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
	/// Use a custom config file
	#[arg(short, long, value_name = "FILE", default_value = "~/.flix")]
	config: PathBuf,

	/// Use a custom database file
	#[arg(short, long, value_name = "DATABASE", default_value = "./flix.db")]
	database: PathBuf,

	/// Enable tracing
	#[arg(short, long)]
	pub trace: bool,

	#[command(subcommand)]
	command: Command,
}

impl Cli {
	pub fn config_path(&self) -> PathBuf {
		fn expect_home_dir() -> PathBuf {
			#[allow(clippy::expect_used)]
			std::env::home_dir().expect("you do not have a home directory")
		}

		match self.config.strip_prefix("~/") {
			Ok(path) => expect_home_dir().join(path),
			Err(_) => self.config.to_owned(),
		}
	}

	pub fn database_path(&self) -> Result<String> {
		self.database
			.as_os_str()
			.to_str()
			.map(ToOwned::to_owned)
			.ok_or_else(|| anyhow!(".as_os_str().to_str()"))
	}

	pub fn command(self) -> Command {
		self.command
	}
}

#[derive(Subcommand)]
pub enum Command {
	/// Initialize a new database
	Init,
	/// Add new items to the database
	Add {
		#[command(subcommand)]
		command: BackendCommand,
	},
	/// Update an existing item in the database
	Update {
		#[command(subcommand)]
		command: BackendCommand,
	},
	/// Delete an existing item in the database
	Delete {
		#[command(subcommand)]
		command: BackendCommand,
	},
	/// Create a toml backup of the database
	Backup {
		/// Change the destination
		#[arg(short, long, value_name = "FILE", default_value = "./flix.toml")]
		output: PathBuf,
	},
	/// Create a database from a toml backup
	Restore {
		/// Change the source
		#[arg(short, long, value_name = "FILE", default_value = "./flix.toml")]
		input: PathBuf,
	},
}

#[derive(Subcommand)]
pub enum BackendCommand {
	/// Use the TMDB backend
	Tmdb {
		#[command(subcommand)]
		command: tmdb::Command,
	},
}

impl From<tmdb::Command> for BackendCommand {
	fn from(value: tmdb::Command) -> Self {
		Self::Tmdb { command: value }
	}
}
