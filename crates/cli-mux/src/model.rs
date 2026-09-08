//! Model for handling media streams.

use std::path::PathBuf;

use serde::Deserialize;

/// Represents a file that was scanned.
#[derive(Debug, Clone)]
pub(crate) struct MediaFile {
	/// The path to the file.
	pub path: PathBuf,
	/// The size of the file.
	pub byte_size: u64,
	/// The streams in the file.
	pub streams: Streams,
}

/// Represents the collection of streams found in a media file.
#[derive(Debug, Clone)]
pub(crate) struct Streams {
	/// The video streams.
	pub video: Vec<VideoStream>,
	/// The audio streams.
	pub audio: Vec<AudioStream>,
	/// The subtitle streams.
	pub subtitle: Vec<SubtitleStream>,
}

/// This trait helps with deserializing `ffmpeg` output.
pub(crate) trait FFStream: serde::de::DeserializeOwned {
	/// The type name that `ffmpeg` uses.
	const FF_TYPE_NAME: &str;
}

/// Represents an individual video stream in a media file.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct VideoStream {
	/// The name of the video codec.
	codec_name: String,
	/// Additional tags on the stream.
	tags: Option<VideoTags>,
}

/// Represents tags on a video stream.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct VideoTags {
	/// The language of the stream.
	language: Option<String>,
}

impl VideoStream {
	/// Get the video codec name.
	pub(crate) fn codec(&self) -> &str {
		&self.codec_name
	}

	/// Get the stream language.
	pub(crate) fn language(&self) -> Option<&str> {
		self.tags.as_ref()?.language.as_deref()
	}
}

impl FFStream for VideoStream {
	const FF_TYPE_NAME: &str = "v";
}

/// Represents an individual audio stream in a media file.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AudioStream {
	/// The name of the audio codec.
	codec_name: String,
	/// Additional tags on the stream.
	tags: Option<AudioTags>,
}

/// Represents tags on an audio stream.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AudioTags {
	/// The language of the stream.
	language: Option<String>,
}

impl AudioStream {
	/// Get the audio codec name.
	pub(crate) fn codec(&self) -> &str {
		&self.codec_name
	}

	/// Get the stream language.
	pub(crate) fn language(&self) -> Option<&str> {
		self.tags.as_ref()?.language.as_deref()
	}
}

impl FFStream for AudioStream {
	const FF_TYPE_NAME: &str = "a";
}

/// Represents an individual subtitle stream in a media file.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SubtitleStream {
	/// The name of the subtitle codec.
	codec_name: String,
	/// Additional tags on the stream.
	tags: Option<SubtitleTags>,
}

/// Represents tags on a subtitle stream.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SubtitleTags {
	/// The language of the stream.
	language: Option<String>,
	/// The title of the stream.
	title: Option<String>,
}

impl SubtitleStream {
	/// Get the subtitle codec name.
	pub(crate) fn codec(&self) -> &str {
		&self.codec_name
	}

	/// Get the stream language.
	pub(crate) fn language(&self) -> Option<&str> {
		self.tags.as_ref()?.language.as_deref()
	}

	/// Get the stream title.
	pub(crate) fn title(&self) -> Option<&str> {
		self.tags.as_ref()?.title.as_deref()
	}
}

impl FFStream for SubtitleStream {
	const FF_TYPE_NAME: &str = "s";
}
