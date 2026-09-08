//! Wrapper around `ffmpeg` to mux media files.

use std::process::Command;

use anyhow::{Context as _, Result};

use crate::model::MediaFile;
use crate::parser::{Matcher, Selector, StreamFlag, StreamType};

/// Helper struct for tracking which output index is currently available.
#[derive(Default)]
struct OutputIndex {
	/// The next free video index.
	video: usize,
	/// The next free audio index.
	audio: usize,
	/// The next free subtitle index.
	subtitle: usize,
}

/// For each input file, apply all selectors and mux the selection.
pub(crate) fn mux_files<T>(
	dry_run: bool,
	files: &[MediaFile],
	selectors: &[Selector],
	fixed_length_start: impl FnOnce(usize) -> T,
	fixed_length_update: impl Fn(&mut T),
	fixed_length_end: impl FnOnce(T),
	print_fn: impl Fn(&T, &str),
) {
	let mut progress = fixed_length_start(files.len());
	for file in files {
		if let Err(err) = mux(dry_run, file, selectors) {
			print_fn(&progress, &format!("{err:?}"));
		}
		fixed_length_update(&mut progress);
	}
	fixed_length_end(progress);
}

/// Apply all selectors and mux the file.
///
/// # Errors
/// If the call to `ffmpeg` fails.
///
/// # Panics
/// If the system fails to allocate a temporary file.
fn mux(dry_run: bool, file: &MediaFile, selectors: &[Selector]) -> Result<()> {
	let mut command = Command::new("ffmpeg");
	let mut command = command.args(["-v", "error"]);

	command = command.arg("-i");
	command = command.arg(file.path.as_os_str());

	for selector in selectors {
		command = command.args(make_map_args(file, selector).with_context(|| {
			let info = if dry_run {
				format!("Streams:\n{file:#?}")
			} else {
				String::from("use -d for stream info")
			};
			format!("Failed to mux {}\n{info}", file.path.display())
		})?);
	}

	command = command.args(["-c:v", "copy", "-c:a", "copy", "-c:s", "mov_text"]);
	command = command.args(["-strict", "-2"]);
	command = command.args(["-movflags", "faststart+disable_chpl+write_colr"]);
	command = command.args(["-map_chapters", "-1"]);
	command = command.args(["-map_metadata", "-1"]);
	command = command.args(["-metadata:g", "encoding_tool=Skrundzflix"]);

	let mut ouput_index = OutputIndex::default();
	for selector in selectors {
		command = command.args(
			make_metadata_args(file, selector, &mut ouput_index).with_context(|| {
				let info = if dry_run {
					format!("Streams:\n{file:#?}")
				} else {
					String::from("use -d for stream info")
				};
				format!("Failed to mux {}\n{info}", file.path.display())
			})?,
		);
	}

	let temp_path = file.path.with_extension("mp4");
	#[expect(clippy::expect_used, reason = "files must have names")]
	{
		command = command.arg(temp_path.file_name().expect("file name exists"));
	}

	if dry_run {
		print_command(command);
	} else {
		let output = command
			.output()
			.with_context(|| format!("Failed to mux {}", file.path.display()))?;

		if !output.status.success() {
			anyhow::bail!(
				"ffmpeg failed for {}:\n\n{}",
				file.path.display(),
				String::from_utf8_lossy(&output.stderr)
			);
		}
	}

	Ok(())
}

/// Generate the `-map` argument list for `ffmpeg`.
///
/// # Errors
/// If the stream selection is unsatisfyable for the file.
fn make_map_args(file: &MediaFile, selector: &Selector) -> Result<Vec<String>> {
	let source_index: u32 = 0;
	let stream_type = selector.stream_type.as_ref();
	let Some(stream_index) = find_stream_index(file, selector) else {
		if selector.optional {
			return Ok(vec![]);
		}

		anyhow::bail!("unsatisfied stream selection");
	};

	Ok(vec![
		String::from("-map"),
		format!("{source_index}:{stream_type}:{stream_index}"),
	])
}

/// Generate the `-metadata` argument list for `ffmpeg`.
///
/// # Errors
/// If the stream selection is unsatisfyable for the file.
fn make_metadata_args(
	file: &MediaFile,
	selector: &Selector,
	index: &mut OutputIndex,
) -> Result<Vec<String>> {
	let stream_type = selector.stream_type.as_ref();
	let stream_language = selector.language();

	// Bail early if the stream is not selected
	let Some(_) = find_stream_index(file, selector) else {
		if selector.optional {
			return Ok(vec![]);
		}

		anyhow::bail!("unsatisfied stream selection");
	};

	let counter = match selector.stream_type {
		StreamType::Video => &mut index.video,
		StreamType::Audio => &mut index.audio,
		StreamType::Subtitle => &mut index.subtitle,
	};
	let stream_index = *counter;
	*counter = counter.saturating_add(1);

	let mut args = vec![
		format!("-metadata:s:{stream_type}:{stream_index}"),
		format!("language={stream_language}"),
	];

	if selector.stream_type == StreamType::Subtitle {
		let sub_title = match selector.flag {
			Some(StreamFlag::Forced) => "Forced",
			Some(StreamFlag::Sdh) => "SDH",
			None => match stream_language {
				"eng" => "English",
				"jpn" => "Japanese",
				_ => anyhow::bail!("Unhandled subtitle language: {stream_language}"),
			},
		};

		args.push(format!("-metadata:s:s:{stream_index}"));
		args.push(format!("title={sub_title}"));
	}

	Ok(args)
}

/// Walk the streams of a file to match the given [Selector].
fn find_stream_index(file: &MediaFile, selector: &Selector) -> Option<usize> {
	let needs_forced = selector.flag == Some(StreamFlag::Forced);
	let needs_sdh = selector.flag == Some(StreamFlag::Sdh);

	match selector.stream_type {
		StreamType::Video => match &selector.matcher {
			Matcher::Index(index) => (file.streams.video.len() > *index).then_some(*index),
			Matcher::Language(language) => file
				.streams
				.video
				.iter()
				.enumerate()
				.filter(|(_, c)| c.language() == Some(language.as_str()))
				.map(|(i, _)| i)
				.next(),
			Matcher::Codec(codec) => file
				.streams
				.video
				.iter()
				.enumerate()
				.filter(|(_, c)| c.codec() == codec)
				.map(|(i, _)| i)
				.next(),
		},
		StreamType::Audio => match &selector.matcher {
			Matcher::Index(index) => (file.streams.audio.len() > *index).then_some(*index),
			Matcher::Language(language) => file
				.streams
				.audio
				.iter()
				.enumerate()
				.filter(|(_, c)| c.language() == Some(language.as_str()))
				.map(|(i, _)| i)
				.next(),
			Matcher::Codec(codec) => file
				.streams
				.audio
				.iter()
				.enumerate()
				.filter(|(_, c)| c.codec() == codec)
				.map(|(i, _)| i)
				.next(),
		},
		StreamType::Subtitle => match &selector.matcher {
			Matcher::Index(index) => (file.streams.subtitle.len() > *index).then_some(*index),
			Matcher::Language(language) => {
				file.streams
					.subtitle
					.iter()
					.enumerate()
					.filter(|(_, c)| c.language() == Some(language.as_str()))
					.filter(|(_, c)| {
						c.title()
							.unwrap_or_default()
							.to_ascii_lowercase()
							.contains("forced") == needs_forced
					})
					.filter(|(_, c)| {
						c.title()
							.unwrap_or_default()
							.to_ascii_lowercase()
							.contains("sdh") == needs_sdh
					})
					.map(|(i, _)| i)
					.next()
			}
			Matcher::Codec(codec) => file
				.streams
				.subtitle
				.iter()
				.enumerate()
				.filter(|(_, c)| c.codec() == codec)
				.map(|(i, _)| i)
				.next(),
		},
	}
}

/// Print out the command to stdout.
#[expect(clippy::print_stdout, reason = "the purpose is to print")]
fn print_command(cmd: &Command) {
	let program = cmd.get_program().to_string_lossy();

	let args = cmd
		.get_args()
		.map(|a| shell_escape(a.to_string_lossy().as_ref()))
		.collect::<Vec<_>>()
		.join(" ");

	println!("{program} {args}");
}

/// Escape a string to be used by a shell.
fn shell_escape(s: &str) -> String {
	if s.chars()
		.all(|c| c.is_ascii_alphanumeric() || "-_./".contains(c))
	{
		s.to_owned()
	} else {
		format!("{s:?}")
	}
}
