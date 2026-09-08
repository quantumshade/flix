//! Helpers for using `ffprobe` to gather stream information from media files.

use std::os::unix::fs::MetadataExt as _;
use std::process::Command;

use anyhow::{Context as _, Result};
use serde::Deserialize;
use walkdir::DirEntry;

use crate::model::{FFStream, MediaFile, Streams};

/// The top-level containing type for `ffprobe` output.
#[derive(Debug, Deserialize)]
struct FFProbeOutput<S> {
	/// The list of returned streams.
	streams: Vec<S>,
}

/// Calls `ffprobe` on a [`DirEntry`] to gather one type of stream.
///
/// # Errors
/// Forwards any IO or parsing errors.
///
/// # Panics
/// If the file path is not utf8.
fn probe_file_streams<S: FFStream>(entry: &DirEntry) -> Result<Vec<S>> {
	let output = Command::new("ffprobe")
		.args([
			"-v",
			"error",
			"-output_format",
			"json",
			"-show_streams",
			"-select_streams",
			S::FF_TYPE_NAME,
			#[expect(clippy::expect_used, reason = "panic if path is not utf8")]
			entry.path().to_str().expect("path should be utf8"),
		])
		.output()
		.with_context(|| format!("Failed to run ffprobe on {}", entry.path().display()))?;

	if !output.status.success() {
		anyhow::bail!(
			"ffprobe failed for {}:\n\n{}",
			entry.path().display(),
			String::from_utf8_lossy(&output.stderr)
		);
	}

	let parsed: FFProbeOutput<S> = serde_json::from_slice(&output.stdout)?;
	Ok(parsed.streams)
}

/// Calls `ffprobe` on a [`DirEntry`] to detect all streams.
///
/// # Errors
/// Forwards any IO or parsing errors.
pub(crate) fn probe_file(entry: &DirEntry) -> Result<MediaFile> {
	Ok(MediaFile {
		path: entry.path().to_path_buf(),
		byte_size: entry.metadata()?.size(),
		streams: Streams {
			video: probe_file_streams(entry)?,
			audio: probe_file_streams(entry)?,
			subtitle: probe_file_streams(entry)?,
		},
	})
}
