//! The generic scanner will scan a directory and automatically
//! detect the type of media, deferring to the correct scanner.

use std::ffi::OsStr;
use std::path::Path;
use std::sync::OnceLock;

use flix_model::id::{CollectionId, MovieId, RawId, ShowId};
use flix_model::numbers::{EpisodeNumbers, SeasonNumber};

use async_stream::stream;
use regex::Regex;
use tokio::fs;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReadDirStream;

use crate::Error;
use crate::scanner::{collection, movie, show};

static MEDIA_FOLDER_REGEX: OnceLock<Regex> = OnceLock::new();
static SEASON_FOLDER_REGEX: OnceLock<Regex> = OnceLock::new();

/// A collection item
pub type Item = crate::Item<Scanner>;

/// The scanner for collections
pub enum Scanner {
	/// A scanned collection
	Collection {
		/// The ID of the parent collection (if any)
		parent: Option<CollectionId>,
		/// The ID of the collection
		id: CollectionId,
		/// The file name of the poster file
		poster_file_name: Option<String>,
	},

	/// A scanned movie
	Movie {
		/// The ID of the parent collection (if any)
		parent: Option<CollectionId>,
		/// The ID of the movie
		id: MovieId,
		/// The file name of the media file
		media_file_name: String,
		/// The file name of the poster file
		poster_file_name: Option<String>,
	},

	/// A scanned show
	Show {
		/// The ID of the parent collection (if any)
		parent: Option<CollectionId>,
		/// The ID of the show
		id: ShowId,
		/// The file name of the poster file
		poster_file_name: Option<String>,
	},
	/// A scanned episode
	Season {
		/// The ID of the show this season belongs to
		show: ShowId,
		/// The season this episode belongs to
		season: SeasonNumber,
		/// The file name of the poster file
		poster_file_name: Option<String>,
	},
	/// A scanned episode
	Episode {
		/// The ID of the show this episode belongs to
		show: ShowId,
		/// The season this episode belongs to
		season: SeasonNumber,
		/// The number(s) of this episode
		episode: EpisodeNumbers,
		/// The file name of the media file
		media_file_name: String,
		/// The file name of the poster file
		poster_file_name: Option<String>,
	},
}

impl From<collection::Scanner> for Scanner {
	fn from(value: collection::Scanner) -> Self {
		match value {
			collection::Scanner::Collection {
				parent,
				id,
				poster_file_name,
			} => Self::Collection {
				parent,
				id,
				poster_file_name,
			},
			collection::Scanner::Movie {
				parent,
				id,
				media_file_name,
				poster_file_name,
			} => Self::Movie {
				parent,
				id,
				media_file_name,
				poster_file_name,
			},
			collection::Scanner::Show {
				parent,
				id,
				poster_file_name,
			} => Self::Show {
				parent,
				id,
				poster_file_name,
			},
			collection::Scanner::Season {
				show,
				season,
				poster_file_name,
			} => Self::Season {
				show,
				season,
				poster_file_name,
			},
			collection::Scanner::Episode {
				show,
				season,
				episode,
				media_file_name,
				poster_file_name,
			} => Self::Episode {
				show,
				season,
				episode,
				media_file_name,
				poster_file_name,
			},
		}
	}
}

impl From<movie::Scanner> for Scanner {
	fn from(value: movie::Scanner) -> Self {
		match value {
			movie::Scanner::Movie {
				parent,
				id,
				media_file_name,
				poster_file_name,
			} => Self::Movie {
				parent,
				id,
				media_file_name,
				poster_file_name,
			},
		}
	}
}

impl From<show::Scanner> for Scanner {
	fn from(value: show::Scanner) -> Self {
		match value {
			show::Scanner::Show {
				parent,
				id,
				poster_file_name,
			} => Self::Show {
				parent,
				id,
				poster_file_name,
			},
			show::Scanner::Season {
				show,
				season,
				poster_file_name,
			} => Self::Season {
				show,
				season,
				poster_file_name,
			},
			show::Scanner::Episode {
				show,
				season,
				episode,
				media_file_name,
				poster_file_name,
			} => Self::Episode {
				show,
				season,
				episode,
				media_file_name,
				poster_file_name,
			},
		}
	}
}

impl Scanner {
	/// Detect the type of a folder and call the correct scanner. Use
	/// this only for detecting possibly ambiguous media:
	///   - Collections
	///   - Movies
	///   - Shows
	pub fn scan_detect_folder(
		path: &Path,
		parent: Option<CollectionId>,
	) -> impl Stream<Item = Item> {
		enum MediaType {
			Collection,
			Movie,
			Show,
		}

		let media_folder_re = MEDIA_FOLDER_REGEX.get_or_init(|| {
			Regex::new(r"^[[[:alnum:]]' -]+ \([[:digit:]]+\) \[[[:digit:]]+\]$")
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

			let Some(Ok(id)) = dir_name
				.split_once('[')
				.and_then(|(_, s)| s.split_once(']'))
				.map(|(s, _)| s.parse::<RawId>())
			else {
				yield Item {
					path: path.to_owned(),
					event: Err(Error::UnexpectedFolder),
				};
				return;
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
					for await event in collection::Scanner::scan_collection(
						path,
						parent,
						CollectionId::from_raw(id),
					) {
						yield event.map(|e| e.into());
					}
				}
				MediaType::Movie => {
					for await event in
						movie::Scanner::scan_movie(path, parent, MovieId::from_raw(id))
					{
						yield event.map(|e| e.into());
					}
				}
				MediaType::Show => {
					for await event in show::Scanner::scan_show(path, parent, ShowId::from_raw(id))
					{
						yield event.map(|e| e.into());
					}
				}
			}
		})
	}
}
