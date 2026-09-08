//! Command line argument parsing.

use std::path::PathBuf;

use crate::parser::Selector;

/// Command line argument parser struct.
#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
	/// Dry run and print commands.
	#[arg(short, long)]
	dry_run: bool,

	/// Stream selectors.
	#[arg(
		short,
		long,
		required = true,
		value_name = "SEL",
		value_delimiter = ';'
	)]
	selectors: Vec<Selector>,

	/// The path to the directory to scan.
	#[arg(value_name = "DIR")]
	scan_dir: PathBuf,
}

impl Args {
	/// True if the user requested a dry run.
	pub(crate) const fn is_dry_run(&self) -> bool {
		self.dry_run
	}

	/// Returns a slice of provided [Selector].
	pub(crate) fn selectors(&self) -> &[Selector] {
		&self.selectors
	}

	/// Get the path to scan.
	///
	/// # Panics
	/// If the user doesn't have a home directory.
	pub(crate) fn scan_dir_path(&self) -> PathBuf {
		self.scan_dir.strip_prefix("~/").map_or_else(
			|_| self.scan_dir.clone(),
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
}
