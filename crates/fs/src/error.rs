use std::io;

/// The error type for filesystem scanning operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// fs::read_dir failed
	#[error("fs::read_dir: {0}")]
	ReadDir(io::Error),
	/// fs::read_dir::next_entry failed
	#[error("fs::read_dir::next_entry: {0}")]
	ReadDirEntry(io::Error),
	/// fs::read_dir::file_type failed
	#[error("fs::read_dir::file_type: {0}")]
	FileType(io::Error),

	/// There is an unexpected file in the directory
	#[error("unexpected file")]
	UnexpectedFile,
	/// There is an unexpected folder in the directory
	#[error("unexpected folder")]
	UnexpectedFolder,
	/// There is an unexpected non-file item in the directory
	#[error("unexpected non-file")]
	UnexpectedNonFile,

	/// There are multiple media files in the directory
	#[error("duplicate media file")]
	DuplicateMediaFile,
	/// There are multiple poster files in the directory
	#[error("duplicate poster file")]
	DuplicatePosterFile,

	/// The directory contains incomplete flix media
	#[error("incomplete")]
	Incomplete,
	/// Some data is inconsistent with the folder structure
	#[error("inconsistent")]
	Inconsistent,
}
