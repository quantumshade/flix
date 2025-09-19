//! The episode scanner will scan a folder and exit

use std::ffi::OsStr;
use std::path::Path;

use flix_model::id::ShowId;
use flix_model::numbers::{EpisodeNumbers, SeasonNumber};

use async_stream::stream;
use tokio::fs;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReadDirStream;

use crate::Error;
use crate::macros::{is_image_extension, is_media_extension};

/// An episode item
pub type Item = crate::Item<Scanner>;

/// The scanner for epispdes
pub enum Scanner {
	/// A scanned episode
	Episode {
		/// The ID of the show this episode belongs to
		show: ShowId,
		/// The season this episode belongs to
		season: SeasonNumber,
		/// The number(s) of this episode
		number: EpisodeNumbers,
		/// The file name of the media file
		media_file_name: String,
		/// The file name of the poster file
		poster_file_name: Option<String>,
	},
}

impl Scanner {
	/// Scan a folder for an episode
	pub fn scan_episode(
		path: &Path,
		show: ShowId,
		season: SeasonNumber,
		number: EpisodeNumbers,
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

			let mut media_file_name = None;
			let mut poster_file_name = None;

			for await dir in ReadDirStream::new(dirs) {
				match dir {
					Ok(dir) => {
						let filetype = match dir.file_type().await {
							Ok(filetype) => filetype,
							Err(err) => {
								yield Item {
									path: path.to_owned(),
									event: Err(Error::FileType(err)),
								};
								continue;
							}
						};
						if !filetype.is_file() {
							yield Item {
								path: path.to_owned(),
								event: Err(Error::UnexpectedNonFile),
							};
							continue;
						}

						let path = dir.path();
						match path.extension().and_then(OsStr::to_str) {
							is_media_extension!() => {
								if media_file_name.is_some() {
									yield Item {
										path: path.to_owned(),
										event: Err(Error::DuplicateMediaFile),
									};
									continue;
								}
								media_file_name = path
									.file_name()
									.and_then(|s| s.to_str())
									.map(ToOwned::to_owned);
								continue;
							}
							is_image_extension!() => {
								if poster_file_name.is_some() {
									yield Item {
										path: path.to_owned(),
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
									path: path.to_owned(),
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

			let Some(media_file_name) = media_file_name else {
				yield Item {
					path: path.to_owned(),
					event: Err(Error::Incomplete),
				};
				return;
			};

			yield Item {
				path: path.to_owned(),
				event: Ok(Self::Episode {
					show,
					season,
					number,
					media_file_name,
					poster_file_name,
				}),
			};
		})
	}
}
