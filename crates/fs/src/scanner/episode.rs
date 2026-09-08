//! The episode scanner will scan a folder and exit.

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
use crate::scanner::{EpisodeScan, MediaRef};

/// An episode item.
pub type Item = crate::Item<Scanner>;

/// The scanner for epispdes.
#[derive(Debug)]
#[expect(clippy::exhaustive_enums, reason = "unlikely to change")]
pub enum Scanner {
	/// A scanned episode.
	Episode(EpisodeScan),
}

impl Scanner {
	/// Scan a folder for an episode.
	#[inline]
	#[expect(clippy::allow_attributes, reason = "for tail_expr_drop_order")]
	#[allow(tail_expr_drop_order, reason = "bug in stream! macro")]
	#[expect(clippy::semicolon_if_nothing_returned, reason = "false positive")]
	pub fn scan_episode(
		path: &Path,
		show_ref: MediaRef<ShowId>,
		season: SeasonNumber,
		episode: EpisodeNumbers,
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
						#[expect(clippy::filetype_is_file, reason = "we only want regular files")]
						if !filetype.is_file() {
							yield Item {
								path,
								event: Err(Error::UnexpectedNonFile),
							};
							continue;
						}

						match path.extension().and_then(OsStr::to_str) {
							is_media_extension!() => {
								if media_file_name.is_some() {
									yield Item {
										path,
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

			let Some(media_file_name) = media_file_name else {
				yield Item {
					path: path.to_owned(),
					event: Err(Error::Incomplete),
				};
				return;
			};

			yield Item {
				path: path.to_owned(),
				event: Ok(Self::Episode(EpisodeScan {
					show_ref,
					season,
					episode,
					media_file_name,
					poster_file_name,
				})),
			};
		})
	}
}
