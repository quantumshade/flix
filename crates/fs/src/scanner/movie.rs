//! The movie scanner will scan a folder and exit

use std::ffi::OsStr;
use std::path::Path;

use flix_model::id::{CollectionId, MovieId};

use async_stream::stream;
use tokio::fs;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReadDirStream;

use crate::Error;
use crate::macros::{is_image_extension, is_media_extension};

/// An movie item
pub type Item = crate::Item<Scanner>;

/// The scanner for movies
pub enum Scanner {
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
}

impl Scanner {
	/// Scan a folder for a movie
	pub fn scan_movie(
		path: &Path,
		parent: Option<CollectionId>,
		id: MovieId,
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
				event: Ok(Self::Movie {
					parent,
					id,
					media_file_name,
					poster_file_name,
				}),
			};
		})
	}
}
