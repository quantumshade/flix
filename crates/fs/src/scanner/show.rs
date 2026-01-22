//! The show scanner will scan a folder and its children

use std::ffi::OsStr;
use std::path::Path;

use flix_model::id::{CollectionId, ShowId};
use flix_model::numbers::SeasonNumber;

use async_stream::stream;
use tokio::fs;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReadDirStream;

use crate::Error;
use crate::macros::is_image_extension;
use crate::scanner::{EpisodeScan, MediaRef, SeasonScan, ShowScan, season};

/// A show item
pub type Item = crate::Item<Scanner>;

/// The scanner for shows
pub enum Scanner {
	/// A scanned show
	Show(ShowScan),
	/// A scanned season
	Season(SeasonScan),
	/// A scanned episode
	Episode(EpisodeScan),
}

impl From<season::Scanner> for Scanner {
	fn from(value: season::Scanner) -> Self {
		match value {
			season::Scanner::Season(s) => Self::Season(s),
			season::Scanner::Episode(e) => Self::Episode(e),
		}
	}
}

impl Scanner {
	/// Scan a folder for a show and its seasons/episodes
	pub fn scan_show(
		path: &Path,
		parent_ref: Option<MediaRef<CollectionId>>,
		id_ref: MediaRef<ShowId>,
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
				event: Ok(Self::Show(ShowScan {
					parent_ref,
					id_ref: id_ref.clone(),
					poster_file_name,
				})),
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

				for await event in
					season::Scanner::scan_season(&season_dir, id_ref.clone(), season_number)
				{
					yield event.map(|e| e.into());
				}
			}
		})
	}
}
