//! The library scanner will scan an entire directory using the generic
//! scanner

use std::ffi::OsStr;
use std::path::Path;

use async_stream::stream;
use tokio::fs;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReadDirStream;

use crate::Error;
use crate::scanner::generic;

/// A library item
pub type Item = crate::Item<generic::Scanner>;

/// The scanner for collections
pub enum Scanner {}

impl Scanner {
	/// Scan a folder for a library
	pub fn scan_library(path: &Path) -> impl Stream<Item = Item> {
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

			let mut subdirs_to_scan = Vec::new();

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

						let path = dir.path();
						if filetype.is_dir() {
							subdirs_to_scan.push(path);
							continue;
						}

						match path.extension().and_then(OsStr::to_str) {
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

			for subdir in subdirs_to_scan {
				for await event in generic::Scanner::scan_detect_folder(&subdir, None) {
					yield event;
				}
			}
		})
	}
}
