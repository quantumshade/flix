//! The generic scanner will scan a directory and automatically
//! detect the type of media, deferring to the correct scanner.

use std::ffi::OsStr;
use std::path::Path;
use std::sync::OnceLock;

use flix_model::id::{CollectionId, MovieId, RawId, ShowId};

use async_stream::stream;
use either::Either;
use regex::Regex;
use tokio::fs;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReadDirStream;

use crate::Error;
use crate::scanner::{
	CollectionScan, EpisodeScan, MediaRef, MovieScan, SeasonScan, ShowScan, collection, movie, show,
};

static MEDIA_FOLDER_REGEX: OnceLock<Regex> = OnceLock::new();
static SEASON_FOLDER_REGEX: OnceLock<Regex> = OnceLock::new();

/// A collection item
pub type Item = crate::Item<Scanner>;

/// The scanner for collections
#[derive(Debug)]
pub enum Scanner {
	/// A scanned collection
	Collection(CollectionScan),
	/// A scanned movie
	Movie(MovieScan),
	/// A scanned show
	Show(ShowScan),
	/// A scanned episode
	Season(SeasonScan),
	/// A scanned episode
	Episode(EpisodeScan),
}

impl From<collection::Scanner> for Scanner {
	fn from(value: collection::Scanner) -> Self {
		match value {
			collection::Scanner::Collection(c) => Self::Collection(c),
			collection::Scanner::Movie(m) => Self::Movie(m),
			collection::Scanner::Show(s) => Self::Show(s),
			collection::Scanner::Season(s) => Self::Season(s),
			collection::Scanner::Episode(e) => Self::Episode(e),
		}
	}
}

impl From<movie::Scanner> for Scanner {
	fn from(value: movie::Scanner) -> Self {
		match value {
			movie::Scanner::Movie(m) => Self::Movie(m),
		}
	}
}

impl From<show::Scanner> for Scanner {
	fn from(value: show::Scanner) -> Self {
		match value {
			show::Scanner::Show(s) => Self::Show(s),
			show::Scanner::Season(s) => Self::Season(s),
			show::Scanner::Episode(e) => Self::Episode(e),
		}
	}
}

impl Scanner {
	/// Helper function for stripping allowed numerical prefixes for sorting ("01 - ")
	fn strip_numeric_prefix(mut s: &str) -> &str {
		while let Some('0'..='9') = s.chars().next() {
			s = &s[1..]
		}
		s.strip_prefix(" - ").unwrap_or(s)
	}

	/// Detect the type of a folder and call the correct scanner. Use
	/// this only for detecting possibly ambiguous media:
	///   - Collections
	///   - Movies
	///   - Shows
	pub fn scan_detect_folder(
		path: &Path,
		parent: Option<MediaRef<CollectionId>>,
	) -> impl Stream<Item = Item> {
		enum MediaType {
			Collection,
			Movie,
			Show,
		}

		let media_folder_re = MEDIA_FOLDER_REGEX.get_or_init(|| {
			Regex::new(r"^[[[:alnum:]]' -]+ \([[:digit:]]+\)( \[[[:digit:]]+\])?$")
				.unwrap_or_else(|err| panic!("regex is invalid: {err}"))
		});
		let season_folder_re = SEASON_FOLDER_REGEX.get_or_init(|| {
			Regex::new(r"^S[[:digit:]]+$").unwrap_or_else(|err| panic!("regex is invalid: {err}"))
		});

		stream!({
			let Some(dir_name) = path.file_name().and_then(OsStr::to_str) else {
				yield Item {
					path: path.to_owned(),
					event: Err(Error::UnexpectedFolder),
				};
				return;
			};

			let dir_name = Self::strip_numeric_prefix(dir_name);

			// Use the explicit ID ("[X]") if it exists, otherwise parse the folder name
			let media_id = if let Some((id_str, _)) = dir_name
				.split_once('[')
				.and_then(|(_, s)| s.split_once(']'))
			{
				let Ok(id) = id_str.parse::<RawId>() else {
					yield Item {
						path: path.to_owned(),
						event: Err(Error::UnexpectedFolder),
					};
					return;
				};
				Either::Left(id)
			} else {
				Either::Right(flix_model::text::normalize_fs_name(dir_name))
			};

			let media_type: MediaType;
			if media_folder_re.is_match(dir_name) {
				let dirs = match fs::read_dir(path).await {
					Ok(dirs) => dirs,
					Err(err) => {
						yield Item {
							path: path.to_owned(),
							event: Err(Error::ReadDir(err)),
						};
						return;
					}
				};

				let mut is_show = false;

				for await dir in ReadDirStream::new(dirs) {
					match dir {
						Ok(dir) => {
							let path = dir.path();

							let filetype = match dir.file_type().await {
								Ok(filetype) => filetype,
								Err(err) => {
									yield Item {
										path,
										event: Err(Error::FileType(err)),
									};
									continue;
								}
							};
							if !filetype.is_dir() {
								continue;
							}

							let Some(folder_name) = path.file_name().and_then(OsStr::to_str) else {
								yield Item {
									path,
									event: Err(Error::UnexpectedFolder),
								};
								continue;
							};

							if season_folder_re.is_match(folder_name) {
								is_show = true;
								break;
							}
						}
						Err(err) => {
							yield Item {
								path: path.to_owned(),
								event: Err(Error::ReadDirEntry(err)),
							};
						}
					}
				}

				if is_show {
					media_type = MediaType::Show;
				} else {
					media_type = MediaType::Movie;
				}
			} else {
				media_type = MediaType::Collection;
			}

			match media_type {
				MediaType::Collection => {
					let id = match media_id {
						Either::Left(raw) => MediaRef::Id(CollectionId::from_raw(raw)),
						Either::Right(slug) => MediaRef::Slug(slug),
					};

					for await event in collection::Scanner::scan_collection(path, parent, id) {
						yield event.map(|e| e.into());
					}
				}
				MediaType::Movie => {
					let id = match media_id {
						Either::Left(raw) => MediaRef::Id(MovieId::from_raw(raw)),
						Either::Right(slug) => MediaRef::Slug(slug),
					};

					for await event in movie::Scanner::scan_movie(path, parent, id) {
						yield event.map(|e| e.into());
					}
				}
				MediaType::Show => {
					let id = match media_id {
						Either::Left(raw) => MediaRef::Id(ShowId::from_raw(raw)),
						Either::Right(slug) => MediaRef::Slug(slug),
					};

					for await event in show::Scanner::scan_show(path, parent, id) {
						yield event.map(|e| e.into());
					}
				}
			}
		})
	}
}
