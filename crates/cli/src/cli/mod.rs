use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub mod tmdb;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
	/// Use a custom config file [default: ~/.flix]
	#[arg(short, long, value_name = "FILE")]
	config: Option<PathBuf>,

	#[command(subcommand)]
	command: Command,
}

impl Cli {
	pub fn config_path(&self) -> PathBuf {
		match self.config.as_ref() {
			Some(path) => match path.strip_prefix("~/") {
				Ok(path) => expect_home_dir().join(path),
				Err(_) => path.to_owned(),
			},
			None => expect_home_dir().join(".flix"),
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
		#[arg(short, long, value_name = "FILE")]
		output: Option<PathBuf>,

		#[command(subcommand)]
		command: BackendCommand,
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

fn expect_home_dir() -> PathBuf {
	#[allow(clippy::expect_used)]
	home::home_dir().expect("you do not have a home directory")
}
