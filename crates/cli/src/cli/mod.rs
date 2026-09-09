//! Command line argument parsing.

use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use clap::{Args, Parser, Subcommand};

pub(crate) mod flix;
pub(crate) mod tmdb;

/// Command line argument parser struct.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
	/// Use a custom config file.
	#[arg(
		short,
		long,
		value_name = "FILE",
		default_value = "~/.config/flix/config.toml"
	)]
	config: PathBuf,
	/// Use a custom cache file.
	#[arg(short = 'C', long, value_name = "FILE", default_value = "./flix.redb")]
	cache: PathBuf,
	/// Use a custom database file.
	#[arg(short, long, value_name = "FILE", default_value = "./flix.db")]
	database: PathBuf,
	/// Enable tracing.
	#[arg(short, long)]
	trace: bool,
	/// Subcommand to execute.
	#[command(subcommand)]
	command: Command,
}

impl Cli {
	/// Get the config file path.
	///
	/// # Panics
	/// If the user doesn't have a home directory.
	#[inline]
	pub(crate) fn config_path(&self) -> PathBuf {
		self.config.strip_prefix("~/").map_or_else(
			|_| self.config.clone(),
			|path| {
				#[expect(
					clippy::expect_used,
					reason = "without a home directory this program is useless"
				)]
				std::env::home_dir()
					.expect("you do not have a home directory")
					.join(path)
			},
		)
	}

	/// Get the path to the cache file.
	#[inline]
	pub(crate) fn cache_path(&self) -> &Path {
		&self.cache
	}

	/// Get the path to the destination database.
	///
	/// # Errors
	/// If the database path is not utf8.
	#[inline]
	pub(crate) fn database_path(&self) -> Result<String> {
		self.database
			.as_os_str()
			.to_str()
			.map(ToOwned::to_owned)
			.ok_or_else(|| anyhow!(".as_os_str().to_str()"))
	}

	/// Get whether or not the trace flag was enabled.
	#[inline]
	pub(crate) const fn trace(&self) -> bool {
		self.trace
	}

	/// Get the subcommand to run.
	#[inline]
	pub(crate) fn get_command(self) -> Command {
		self.command
	}
}

/// Optional overrides for media information.
#[derive(Args)]
pub(crate) struct AddOverrides {
	/// Override the displayed title.
	#[arg(long)]
	pub title: Option<String>,
	/// Override the displayed overview.
	#[arg(long)]
	pub overview: Option<String>,
	/// Override the title used to sort.
	#[arg(long)]
	pub sort_title: Option<String>,
	/// Override the filesystem slug.
	#[arg(long)]
	pub fs_slug: Option<String>,
	/// Overrride the web slug.
	#[arg(long)]
	pub web_slug: Option<String>,
}

/// Top level cli commands.
#[derive(Subcommand)]
#[expect(clippy::large_enum_variant, reason = "not performance critical")]
pub(crate) enum Command {
	/// Initialize a new database.
	Init,
	/// Add new items to the database.
	Add {
		/// Overrides.
		#[command(flatten)]
		overrides: AddOverrides,
		/// Command.
		#[command(subcommand)]
		command: AddCommand,
	},
}

/// Wrapper for `add` around different backends.
#[derive(Subcommand)]
pub(crate) enum AddCommand {
	/// Use the flix backend.
	Flix {
		/// Command backend.
		#[command(subcommand)]
		command: flix::AddCommand,
	},
	/// Use the TMDB backend.
	Tmdb {
		/// Command backend.
		#[command(subcommand)]
		command: tmdb::Command,
	},
}

impl From<flix::AddCommand> for AddCommand {
	#[inline]
	fn from(value: flix::AddCommand) -> Self {
		Self::Flix { command: value }
	}
}

impl From<tmdb::Command> for AddCommand {
	#[inline]
	fn from(value: tmdb::Command) -> Self {
		Self::Tmdb { command: value }
	}
}
