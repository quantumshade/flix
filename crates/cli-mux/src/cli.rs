use std::path::PathBuf;

use clap::Parser;

use crate::parser::Selector;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
	/// Dry run and print commands
	#[arg(short, long)]
	dry_run: bool,

	/// Stream selectors
	#[arg(
		short,
		long,
		required = true,
		value_name = "SEL",
		value_delimiter = ';'
	)]
	selectors: Vec<Selector>,

	/// The path to the directory to scan
	#[arg(value_name = "DIR")]
	scan_dir: PathBuf,
}

impl Cli {
	pub fn is_dry(&self) -> bool {
		self.dry_run
	}

	pub fn selectors(&self) -> &[Selector] {
		&self.selectors
	}

	pub fn scan_dir_path(&self) -> PathBuf {
		fn expect_home_dir() -> PathBuf {
			#[allow(clippy::expect_used)]
			std::env::home_dir().expect("you do not have a home directory")
		}

		match self.scan_dir.strip_prefix("~/") {
			Ok(path) => expect_home_dir().join(path),
			Err(_) => self.scan_dir.to_owned(),
		}
	}
}
