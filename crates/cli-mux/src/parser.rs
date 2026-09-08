//! Parsing types for `ffmpeg` output.

use core::error::Error;
use core::fmt;
use core::str::FromStr;

/// The type of a stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StreamType {
	/// A video stream.
	Video,
	/// An audio stream.
	Audio,
	/// A subtitle stream.
	Subtitle,
}

impl AsRef<str> for StreamType {
	fn as_ref(&self) -> &str {
		match self {
			Self::Video => "v",
			Self::Audio => "a",
			Self::Subtitle => "s",
		}
	}
}

impl FromStr for StreamType {
	type Err = ParseSelectorError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"v" => Ok(Self::Video),
			"a" => Ok(Self::Audio),
			"s" => Ok(Self::Subtitle),
			other => Err(ParseSelectorError::InvalidStreamType(other.to_owned())),
		}
	}
}

/// A property of a stream to match on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Matcher {
	/// Match a specific stream index.
	Index(usize),
	/// Match the first stream of the given language.
	Language(String),
	/// Match the first stream of the given codec.
	Codec(String),
}

impl FromStr for Matcher {
	type Err = ParseSelectorError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Ok(idx) = s.parse::<usize>() {
			return Ok(Self::Index(idx));
		}

		if s.len() == 3 {
			return Ok(Self::Language(s.to_ascii_lowercase()));
		}
		Ok(Self::Codec(s.to_ascii_lowercase()))
	}
}

/// Flags applied to streams.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StreamFlag {
	/// Forced flag.
	Forced,
	/// SDH flag.
	Sdh,
}

impl FromStr for StreamFlag {
	type Err = ParseSelectorError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_str() {
			"forced" => Ok(Self::Forced),
			"sdh" => Ok(Self::Sdh),
			other => Err(ParseSelectorError::InvalidFlag(other.to_owned())),
		}
	}
}

/// A complete selector which has everything it needs to find a specific stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Selector {
	/// The type of the stream to select.
	pub stream_type: StreamType,
	/// The matcher to use when selecting.
	pub matcher: Matcher,
	/// A desired flag.
	pub flag: Option<StreamFlag>,
	/// Whether or not to bail if the stream cannot be found.
	pub optional: bool,
	/// The output language of the stream.
	pub out_lang: String,
}

impl Selector {
	/// Get the target language of the selector.
	pub(crate) fn language(&self) -> &str {
		&self.out_lang
	}
}

impl FromStr for Selector {
	type Err = ParseSelectorError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		if input.is_empty() {
			return Err(ParseSelectorError::Empty);
		}

		let (input, optional) = input
			.strip_suffix('?')
			.map_or((input, false), |s| (s, true));

		let (left, out_lang) = input.split_once('>').unwrap_or((input, ""));

		let mut parts = left.splitn(3, ':');

		let stream_type: StreamType = parts
			.next()
			.ok_or(ParseSelectorError::InvalidFormat)?
			.parse()?;

		let matcher: Matcher = parts
			.next()
			.ok_or(ParseSelectorError::InvalidFormat)?
			.parse()?;

		let flag = parts
			.next()
			.filter(|s| !s.is_empty())
			.map(str::parse)
			.transpose()?;

		let out_lang = match (out_lang.is_empty(), &matcher) {
			(false, _) => out_lang.to_owned(),
			(true, Matcher::Language(language)) => language.clone(),
			(true, _) => return Err(ParseSelectorError::UnspecifiedLanguage),
		};

		Ok(Self {
			stream_type,
			matcher,
			flag,
			optional,
			out_lang,
		})
	}
}

/// Errors that can arise when parsing a selector string.
#[derive(Debug)]
pub(crate) enum ParseSelectorError {
	/// The selector string is empty.
	Empty,
	/// The selector string has an invalid format.
	InvalidFormat,
	/// The requested stream type in invalid.
	InvalidStreamType(String),
	/// The requested flag is invalid.
	InvalidFlag(String),
	/// The target language is unspecified.
	UnspecifiedLanguage,
}

#[expect(clippy::missing_trait_methods, reason = "accept default methods")]
impl Error for ParseSelectorError {}

impl fmt::Display for ParseSelectorError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Empty => write!(f, "selector was empty"),
			Self::InvalidFormat => write!(f, "invalid selector format"),
			Self::InvalidStreamType(s) => write!(f, "invalid stream type '{s}'"),
			Self::InvalidFlag(s) => write!(f, "invalid stream flag '{s}'"),
			Self::UnspecifiedLanguage => write!(f, "unspecified output language"),
		}
	}
}
