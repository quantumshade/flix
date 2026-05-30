use core::error::Error;
use core::fmt;
use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamType {
	Video,
	Audio,
	Subtitle,
}

impl AsRef<str> for StreamType {
	fn as_ref(&self) -> &str {
		match self {
			StreamType::Video => "v",
			StreamType::Audio => "a",
			StreamType::Subtitle => "s",
		}
	}
}

impl FromStr for StreamType {
	type Err = ParseSelectorError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"v" => Ok(StreamType::Video),
			"a" => Ok(StreamType::Audio),
			"s" => Ok(StreamType::Subtitle),
			other => Err(ParseSelectorError::InvalidStreamType(other.to_string())),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Matcher {
	Index(usize),
	Language(String),
	Codec(String),
}

impl FromStr for Matcher {
	type Err = ParseSelectorError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Ok(idx) = s.parse::<usize>() {
			return Ok(Matcher::Index(idx));
		}

		if s.len() == 3 {
			return Ok(Matcher::Language(s.to_ascii_lowercase()));
		}
		Ok(Matcher::Codec(s.to_ascii_lowercase()))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamFlag {
	Forced,
	Sdh,
}

impl FromStr for StreamFlag {
	type Err = ParseSelectorError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_str() {
			"forced" => Ok(StreamFlag::Forced),
			"sdh" => Ok(StreamFlag::Sdh),
			other => Err(ParseSelectorError::InvalidFlag(other.to_string())),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selector {
	pub stream_type: StreamType,
	pub matcher: Matcher,
	pub flag: Option<StreamFlag>,
	pub optional: bool,
	pub out_lang: String,
}

impl Selector {
	pub fn language(&self) -> &str {
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

		Ok(Selector {
			stream_type,
			matcher,
			flag,
			optional,
			out_lang,
		})
	}
}

#[derive(Debug)]
pub enum ParseSelectorError {
	Empty,
	InvalidFormat,
	InvalidStreamType(String),
	InvalidFlag(String),
	UnspecifiedLanguage,
}

impl Error for ParseSelectorError {}

impl fmt::Display for ParseSelectorError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Empty => write!(f, "selector was empty"),
			Self::InvalidFormat => write!(f, "invalid selector format"),
			Self::InvalidStreamType(s) => write!(f, "invalid stream type '{}'", s),
			Self::InvalidFlag(s) => write!(f, "invalid stream flag '{}'", s),
			Self::UnspecifiedLanguage => write!(f, "unspecified output language"),
		}
	}
}
