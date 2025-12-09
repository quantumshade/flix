//! The collection scanner will scan a folder and its children

use core::pin::Pin;
use std::ffi::OsStr;
use std::path::Path;

use flix_model::id::{CollectionId, MovieId, ShowId};
use flix_model::numbers::{EpisodeNumbers, SeasonNumber};

use async_stream::stream;
use tokio::fs;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReadDirStream;

use crate::Error;
use crate::macros::is_image_extension;
use crate::scanner::{generic, movie, show};

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
		/// The number of this season
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

impl From<generic::Scanner> for Scanner {
	fn from(value: generic::Scanner) -> Self {
		match value {
			generic::Scanner::Collection {
				parent,
				id,
				poster_file_name,
			} => Self::Collection {
				parent,
				id,
				poster_file_name,
			},
			generic::Scanner::Movie {
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
			generic::Scanner::Show {
				parent,
				id,
				poster_file_name,
			} => Self::Show {
				parent,
				id,
				poster_file_name,
			},
			generic::Scanner::Season {
				show,
				season,
				poster_file_name,
			} => Self::Season {
				show,
				season,
				poster_file_name,
			},
			generic::Scanner::Episode {
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
	/// Scan a folder for a collection
	pub fn scan_collection(
		path: &Path,
		parent: Option<CollectionId>,
		id: CollectionId,
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
				event: Ok(Self::Collection {
					parent,
					id,
					poster_file_name,
				}),
			};

			for subdir in subdirs_to_scan {
				for await event in generic::Scanner::scan_detect_folder(&subdir, Some(id)) {
					yield event.map(|e| e.into());
				}
			}
		}))
	}
}
