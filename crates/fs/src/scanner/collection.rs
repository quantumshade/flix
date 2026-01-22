//! The collection scanner will scan a folder and its children

use core::pin::Pin;
use std::ffi::OsStr;
use std::path::Path;

use flix_model::id::CollectionId;

use async_stream::stream;
use tokio::fs;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReadDirStream;

use crate::Error;
use crate::macros::is_image_extension;
use crate::scanner::{
	CollectionScan, EpisodeScan, MediaRef, MovieScan, SeasonScan, ShowScan, generic, movie, show,
};

/// A collection item
pub type Item = crate::Item<Scanner>;

/// The scanner for collections
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

impl From<generic::Scanner> for Scanner {
	fn from(value: generic::Scanner) -> Self {
		match value {
			generic::Scanner::Collection(c) => Self::Collection(c),
			generic::Scanner::Movie(m) => Self::Movie(m),
			generic::Scanner::Show(s) => Self::Show(s),
			generic::Scanner::Season(s) => Self::Season(s),
			generic::Scanner::Episode(e) => Self::Episode(e),
		}
	}
}

impl Scanner {
	/// Scan a folder for a collection
	pub fn scan_collection(
		path: &Path,
		parent_ref: Option<MediaRef<CollectionId>>,
		id_ref: MediaRef<CollectionId>,
	) -> Pin<Box<impl Stream<Item = Item>>> {
		Box::pin(stream!({
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
			let mut subdirs_to_scan = Vec::new();

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
							subdirs_to_scan.push(path);
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
				event: Ok(Self::Collection(CollectionScan {
					parent_ref,
					id_ref: id_ref.clone(),
					poster_file_name,
				})),
			};

			for subdir in subdirs_to_scan {
				for await event in
					generic::Scanner::scan_detect_folder(&subdir, Some(id_ref.clone()))
				{
					yield event.map(|e| e.into());
				}
			}
		}))
	}
}
