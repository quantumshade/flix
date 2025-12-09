//! The show scanner will scan a folder and its children

use std::ffi::OsStr;
use std::path::Path;

use flix_model::id::{CollectionId, ShowId};
use flix_model::numbers::{EpisodeNumbers, SeasonNumber};

use async_stream::stream;
use tokio::fs;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReadDirStream;

use crate::Error;
use crate::macros::is_image_extension;
use crate::scanner::season;

/// A show item
pub type Item = crate::Item<Scanner>;

/// The scanner for shows
pub enum Scanner {
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

impl From<season::Scanner> for Scanner {
	fn from(value: season::Scanner) -> Self {
		match value {
			season::Scanner::Season {
				show,
				season,
				poster_file_name,
			} => Self::Season {
				show,
				season,
				poster_file_name,
			},
			season::Scanner::Episode {
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
	/// Scan a folder for a show and its seasons/episodes
	pub fn scan_show(
		path: &Path,
		parent: Option<CollectionId>,
		id: ShowId,
	) -> impl Stream<Item = Item> {
		stream!({
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

			let mut poster_file_name = None;
			let mut season_dirs_to_scan = Vec::new();

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

						if filetype.is_dir() {
							season_dirs_to_scan.push(path);
							continue;
						}

						match path.extension().and_then(OsStr::to_str) {
							is_image_extension!() => {
								if poster_file_name.is_some() {
									yield Item {
										path,
										event: Err(Error::DuplicatePosterFile),
									};
									continue;
								}
								poster_file_name = path
									.file_name()
									.and_then(|s| s.to_str())
									.map(ToOwned::to_owned);
							}
							Some(_) | None => {
								yield Item {
									path,
									event: Err(Error::UnexpectedFile),
								};
							}
						}
					}
					Err(err) => {
						yield Item {
							path: path.to_owned(),
							event: Err(Error::ReadDirEntry(err)),
						}
					}
				}
			}

			yield Item {
				path: path.to_owned(),
				event: Ok(Self::Show {
					parent,
					id,
					poster_file_name,
				}),
			};

			for season_dir in season_dirs_to_scan {
				let Some(season_dir_name) = season_dir.file_name().and_then(OsStr::to_str) else {
					yield Item {
						path: season_dir,
						event: Err(Error::UnexpectedFolder),
					};
					continue;
				};

				let Some(Ok(season_number)) = season_dir_name
					.split_once('S')
					.map(|(_, s)| s.parse::<SeasonNumber>())
				else {
					yield Item {
						path: season_dir,
						event: Err(Error::UnexpectedFolder),
					};
					continue;
				};

				for await event in season::Scanner::scan_season(&season_dir, id, season_number) {
					yield event.map(|e| e.into());
				}
			}
		})
	}
}
