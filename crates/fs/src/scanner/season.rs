//! The episode scanner will scan a folder and its children.

use std::ffi::OsStr;
use std::path::Path;

use flix_model::id::ShowId;
use flix_model::numbers::{EpisodeNumbers, SeasonNumber};

use async_stream::stream;
use tokio::fs;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReadDirStream;

use crate::Error;
use crate::macros::is_image_extension;
use crate::scanner::{EpisodeScan, MediaRef, SeasonScan, episode};

/// A season item.
pub type Item = crate::Item<Scanner>;

/// The scanner for seasons.
#[derive(Debug)]
#[expect(clippy::exhaustive_enums, reason = "unlikely to change")]
pub enum Scanner {
	/// A scanned season.
	Season(SeasonScan),
	/// A scanned episode.
	Episode(EpisodeScan),
}

impl From<episode::Scanner> for Scanner {
	#[inline]
	fn from(value: episode::Scanner) -> Self {
		match value {
			episode::Scanner::Episode(e) => Self::Episode(e),
		}
	}
}

impl Scanner {
	/// Scan a folder for a season and its episodes.
	#[inline]
	#[expect(clippy::allow_attributes, reason = "for tail_expr_drop_order")]
	#[allow(tail_expr_drop_order, reason = "bug in stream! macro")]
	#[expect(clippy::semicolon_if_nothing_returned, reason = "false positive")]
	pub fn scan_season(
		path: &Path,
		show_ref: MediaRef<ShowId>,
		season: SeasonNumber,
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
			let mut episode_dirs_to_scan = Vec::new();

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
							episode_dirs_to_scan.push(path);
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
				event: Ok(Self::Season(SeasonScan {
					show_ref: show_ref.clone(),
					season,
					poster_file_name,
				})),
			};

			for episode_dir in episode_dirs_to_scan {
				let Some(episode_dir_name) = episode_dir.file_name().and_then(OsStr::to_str) else {
					yield Item {
						path: episode_dir,
						event: Err(Error::UnexpectedFolder),
					};
					continue;
				};

				let Some((_, s_e_str)) = episode_dir_name.split_once('S') else {
					yield Item {
						path: episode_dir,
						event: Err(Error::UnexpectedFolder),
					};
					continue;
				};
				let Some((s_str, e_str)) = s_e_str.split_once('E') else {
					yield Item {
						path: episode_dir,
						event: Err(Error::UnexpectedFolder),
					};
					continue;
				};

				let Ok(season_number) = s_str.parse::<SeasonNumber>() else {
					yield Item {
						path: episode_dir,
						event: Err(Error::UnexpectedFolder),
					};
					continue;
				};
				if season_number != season {
					yield Item {
						path: episode_dir,
						event: Err(Error::Inconsistent),
					};
					continue;
				}

				let Ok(episode_numbers) = e_str
					.split('E')
					.map(str::parse)
					.collect::<Result<Vec<_>, _>>()
				else {
					yield Item {
						path: episode_dir,
						event: Err(Error::UnexpectedFolder),
					};
					continue;
				};
				let Ok(episode_numbers) = EpisodeNumbers::try_from(episode_numbers.as_ref()) else {
					yield Item {
						path: episode_dir,
						event: Err(Error::UnexpectedFolder),
					};
					continue;
				};

				for await event in episode::Scanner::scan_episode(
					&episode_dir,
					show_ref.clone(),
					season_number,
					episode_numbers,
				) {
					yield event.map(Into::into);
				}
			}
		})
	}
}
