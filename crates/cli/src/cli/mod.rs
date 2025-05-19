use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub mod tmdb;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
	/// Use a custom config file
	#[arg(short, long, value_name = "FILE", default_value = "~/.flix")]
	config: PathBuf,

	#[command(subcommand)]
	command: Command,
}

impl Cli {
	pub fn config_path(&self) -> PathBuf {
		match self.config.strip_prefix("~/") {
			Ok(path) => expect_home_dir().join(path),
			Err(_) => self.config.to_owned(),
		}
	}

	pub fn command(&self) -> &Command {
		&self.command
	}
}

#[derive(Subcommand)]
pub enum Command {
	/// Print a flix manifest
	Print {
		#[command(subcommand)]
		command: BackendCommand,
	},
	/// Write a flix manifest if the destination does not exist
	Write {
		/// Overwrite the destination
		#[arg(short, long, default_value_t = false)]
		force: bool,

		/// Change the destination
		#[arg(short, long, value_name = "FILE", default_value = "flix.toml")]
		output: PathBuf,

		#[command(subcommand)]
		command: BackendCommand,
	},
	/// Update a flix manifest
	Update {
		/// Change the destination
		#[arg(short, long, value_name = "FILE", default_value = "flix.toml")]
		output: PathBuf,
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

fn expect_home_dir() -> PathBuf {
	#[allow(clippy::expect_used)]
	home::home_dir().expect("you do not have a home directory")
}
