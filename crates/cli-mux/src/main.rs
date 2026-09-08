//! flix-mux.

use core::time::Duration;

use clap::Parser as _;
use console::style;
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};

use crate::mux::mux_files;
use crate::scan::scan_directory;

mod cli;
mod model;
mod mux;
mod parser;
mod probe;
mod scan;

#[cfg_attr(test, expect(clippy::missing_panics_doc, reason = "main function"))]
#[expect(clippy::print_stderr, reason = "we want to print in a binary")]
#[expect(clippy::print_stdout, reason = "we want to print in a binary")]
fn main() {
	let cli = cli::Args::parse();

	let (files, size) = scan_directory(
		&cli.scan_dir_path(),
		|| {
			let progress = ProgressBar::new_spinner();
			progress.set_message(format!("Scanning {}", cli.scan_dir_path().display()));
			progress.enable_steady_tick(Duration::from_millis(50));
			progress
		},
		|progress| progress.finish(),
		|len| {
			let progress = ProgressBar::new(u64::try_from(len).unwrap_or(0));
			progress.set_style(
				#[expect(clippy::expect_used, reason = "panic if the template is invalid")]
				#[expect(clippy::literal_string_with_formatting_args, reason = "template")]
				ProgressStyle::with_template("[{elapsed_precise}] {wide_bar} {pos}/{len} ({msg})")
					.expect("static template"),
			);
			progress
		},
		|progress| progress.inc(1),
		|progress| progress.finish_and_clear(),
		|progress, msg| {
			progress.suspend(|| eprintln!("{} {msg}", style("[WARN]").bold().yellow()));
		},
	);
	println!("Found {} files ({})", files.len(), HumanBytes(size));

	mux_files(
		cli.is_dry_run(),
		&files,
		cli.selectors(),
		|len| {
			let progress = ProgressBar::new(u64::try_from(len).unwrap_or(0));
			progress.set_style(
				#[expect(clippy::expect_used, reason = "panic if the template is invalid")]
				#[expect(clippy::literal_string_with_formatting_args, reason = "template")]
				ProgressStyle::with_template("[{elapsed_precise}] {wide_bar} {pos}/{len} ({msg})")
					.expect("static template"),
			);
			progress.enable_steady_tick(Duration::from_secs(1));
			progress
		},
		|progress| progress.inc(1),
		|progress| progress.finish_with_message("done"),
		|progress, msg| progress.suspend(|| eprintln!("{} {msg}", style("[ERR]").bold().red())),
	);
}
