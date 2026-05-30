use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct MediaFile {
	pub path: PathBuf,
	pub byte_size: u64,
	pub streams: Streams,
}

#[derive(Debug, Clone)]
pub struct Streams {
	pub video: Vec<VideoStream>,
	pub audio: Vec<AudioStream>,
	pub subtitle: Vec<SubtitleStream>,
}

pub trait FFStream: serde::de::DeserializeOwned {
	const FF_TYPE_NAME: &str;
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoStream {
	codec_name: String,
	tags: Option<VideoTags>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoTags {
	language: Option<String>,
}

impl VideoStream {
	pub fn codec(&self) -> &str {
		&self.codec_name
	}

	pub fn language(&self) -> Option<&str> {
		self.tags.as_ref()?.language.as_deref()
	}
}

impl FFStream for VideoStream {
	const FF_TYPE_NAME: &str = "v";
}

#[derive(Debug, Clone, Deserialize)]
pub struct AudioStream {
	codec_name: String,
	tags: Option<AudioTags>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AudioTags {
	language: Option<String>,
}

impl AudioStream {
	pub fn codec(&self) -> &str {
		&self.codec_name
	}

	pub fn language(&self) -> Option<&str> {
		self.tags.as_ref()?.language.as_deref()
	}
}

impl FFStream for AudioStream {
	const FF_TYPE_NAME: &str = "a";
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubtitleStream {
	codec_name: String,
	tags: Option<SubtitleTags>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubtitleTags {
	language: Option<String>,
	title: Option<String>,
}

impl SubtitleStream {
	pub fn codec(&self) -> &str {
		&self.codec_name
	}

	pub fn language(&self) -> Option<&str> {
		self.tags.as_ref()?.language.as_deref()
	}

	pub fn title(&self) -> Option<&str> {
		self.tags.as_ref()?.title.as_deref()
	}
}

impl FFStream for SubtitleStream {
	const FF_TYPE_NAME: &str = "s";
}
