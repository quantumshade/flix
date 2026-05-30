use std::os::unix::fs::MetadataExt;
use std::process::Command;

use anyhow::{Context as _, Result};
use serde::Deserialize;
use walkdir::DirEntry;

use crate::model::{FFStream, MediaFile, Streams};

#[derive(Debug, Deserialize)]
struct FFProbeOutput<S> {
	streams: Vec<S>,
}

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
			#[expect(clippy::expect_used)]
			entry.path().to_str().expect("path should be utf8"),
		])
		.output()
		.with_context(|| format!("Failed to run ffprobe on {:?}", entry.path()))?;

	if !output.status.success() {
		anyhow::bail!(
			"ffprobe failed for {:?}:\n\n{}",
			entry.path(),
			String::from_utf8_lossy(&output.stderr)
		);
	}

	let parsed: FFProbeOutput<S> = serde_json::from_slice(&output.stdout)?;
	Ok(parsed.streams)
}

pub fn probe_file(entry: &DirEntry) -> Result<MediaFile> {
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
