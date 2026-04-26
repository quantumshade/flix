use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use clap::{Args, Parser, Subcommand};

pub mod flix;
pub mod tmdb;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
	/// Use a custom config file
	#[arg(short, long, value_name = "FILE", default_value = "~/.flix")]
	config: PathBuf,

	/// Use a custom cache file
	#[arg(short = 'C', long, value_name = "FILE", default_value = "./flix.redb")]
	cache: PathBuf,

	/// Use a custom database file
	#[arg(short, long, value_name = "FILE", default_value = "./flix.db")]
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

	pub fn cache_path(&self) -> &Path {
		&self.cache
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

#[derive(Args)]
pub struct AddOverrides {
	#[arg(long)]
	pub title: Option<String>,
	#[arg(long)]
	pub sort_title: Option<String>,
	#[arg(long)]
	pub fs_slug: Option<String>,
	#[arg(long)]
	pub web_slug: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
	/// Initialize a new database
	Init,
	/// Add new items to the database
	Add {
		#[command(flatten)]
		overrides: AddOverrides,
		#[command(subcommand)]
		command: AddCommand,
	},
	/// Update an existing item in the database
	Update {
		#[command(subcommand)]
		command: UpdateCommand,
	},
}

#[derive(Subcommand)]
pub enum AddCommand {
	/// Use the flix backend
	Flix {
		#[command(subcommand)]
		command: flix::AddCommand,
	},
	/// Use the TMDB backend
	Tmdb {
		#[command(subcommand)]
		command: tmdb::Command,
	},
}

impl From<flix::AddCommand> for AddCommand {
	fn from(value: flix::AddCommand) -> Self {
		Self::Flix { command: value }
	}
}

impl From<tmdb::Command> for AddCommand {
	fn from(value: tmdb::Command) -> Self {
		Self::Tmdb { command: value }
	}
}

#[derive(Subcommand)]
pub enum UpdateCommand {
	/// Use the TMDB backend
	Tmdb {
		#[command(subcommand)]
		command: tmdb::Command,
	},
}

impl From<tmdb::Command> for UpdateCommand {
	fn from(value: tmdb::Command) -> Self {
		Self::Tmdb { command: value }
	}
}
