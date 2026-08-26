use std::process::Command;

use anyhow::{Context as _, Result};

use crate::model::MediaFile;
use crate::parser::{Matcher, Selector, StreamFlag, StreamType};

#[derive(Default)]
struct OutputIndex {
	video: usize,
	audio: usize,
	subtitle: usize,
}

pub fn mux_files<T>(
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
			print_fn(&progress, &format!("{:?}", err));
		}
		fixed_length_update(&mut progress);
	}
	fixed_length_end(progress);
}

#[expect(clippy::expect_used)]
fn mux(dry_run: bool, file: &MediaFile, selectors: &[Selector]) -> Result<()> {
	let mut command = Command::new("ffmpeg");
	let mut command = command.args(["-v", "error"]);

	command = command.arg("-i");
	command = command.arg(file.path.as_os_str());

	for selector in selectors {
		command = command.args(
			make_map_args(file, selector)
				.with_context(|| format!("Failed to mux {:?}", file.path))?,
		);
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
			make_metadata_args(file, selector, &mut ouput_index)
				.with_context(|| format!("Failed to mux {:?}", file.path))?,
		);
	}

	let temp_path = file.path.with_extension("mp4");
	command = command.arg(temp_path.file_name().expect("file name exists"));

	if dry_run {
		print_command(command);
	} else {
		let output = command
			.output()
			.with_context(|| format!("Failed to mux {:?}", file.path))?;

		if !output.status.success() {
			anyhow::bail!(
				"ffmpeg failed for {:?}:\n\n{}",
				file.path,
				String::from_utf8_lossy(&output.stderr)
			);
		}
	}

	Ok(())
}

fn make_map_args(file: &MediaFile, selector: &Selector) -> Result<Vec<String>> {
	let source_index = 0;
	let stream_type = selector.stream_type.as_ref();
	let Some(stream_index) = find_stream_index(file, selector) else {
		if selector.optional {
			return Ok(vec![]);
		} else {
			anyhow::bail!("unsatisfied stream selection");
		}
	};

	Ok(vec![
		String::from("-map"),
		format!("{}:{}:{}", source_index, stream_type, stream_index),
	])
}

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
		} else {
			anyhow::bail!("unsatisfied stream selection");
		}
	};

	let counter = match selector.stream_type {
		StreamType::Video => &mut index.video,
		StreamType::Audio => &mut index.audio,
		StreamType::Subtitle => &mut index.subtitle,
	};
	let stream_index = *counter;
	*counter = counter.saturating_add(1);

	let mut args = vec![
		format!("-metadata:s:{}:{}", stream_type, stream_index),
		format!("language={}", stream_language),
	];

	if selector.stream_type == StreamType::Subtitle {
		let sub_title = match selector.flag {
			Some(StreamFlag::Forced) => "Forced",
			Some(StreamFlag::Sdh) => "SDH",
			None => match stream_language {
				"eng" => "English",
				"jpn" => "Japanese",
				_ => anyhow::bail!("Unhandled subtitle language: {}", stream_language),
			},
		};

		args.push(format!("-metadata:s:s:{}", stream_index));
		args.push(format!("title={}", sub_title));
	}

	Ok(args)
}

fn find_stream_index(file: &MediaFile, selector: &Selector) -> Option<usize> {
	let needs_forced = selector.flag == Some(StreamFlag::Forced);
	let needs_sdh = selector.flag == Some(StreamFlag::Sdh);

	match selector.stream_type {
		StreamType::Video => match selector.matcher {
			Matcher::Index(index) => (file.streams.video.len() > index).then_some(index),
			Matcher::Language(ref language) => file
				.streams
				.video
				.iter()
				.enumerate()
				.filter(|(_, c)| c.language() == Some(language.as_str()))
				.map(|(i, _)| i)
				.next(),
			Matcher::Codec(ref codec) => file
				.streams
				.video
				.iter()
				.enumerate()
				.filter(|(_, c)| c.codec() == codec)
				.map(|(i, _)| i)
				.next(),
		},
		StreamType::Audio => match selector.matcher {
			Matcher::Index(index) => (file.streams.audio.len() > index).then_some(index),
			Matcher::Language(ref language) => file
				.streams
				.audio
				.iter()
				.enumerate()
				.filter(|(_, c)| c.language() == Some(language.as_str()))
				.map(|(i, _)| i)
				.next(),
			Matcher::Codec(ref codec) => file
				.streams
				.audio
				.iter()
				.enumerate()
				.filter(|(_, c)| c.codec() == codec)
				.map(|(i, _)| i)
				.next(),
		},
		StreamType::Subtitle => match selector.matcher {
			Matcher::Index(index) => (file.streams.subtitle.len() > index).then_some(index),
			Matcher::Language(ref language) => {
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
			Matcher::Codec(ref codec) => file
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

fn print_command(cmd: &Command) {
	let program = cmd.get_program().to_string_lossy();

	let args = cmd
		.get_args()
		.map(|a| shell_escape(a.to_string_lossy().as_ref()))
		.collect::<Vec<_>>()
		.join(" ");

	println!("{} {}", program, args);
}

fn shell_escape(s: &str) -> String {
	if s.chars()
		.all(|c| c.is_ascii_alphanumeric() || "-_./".contains(c))
	{
		s.to_string()
	} else {
		format!("{:?}", s)
	}
}
