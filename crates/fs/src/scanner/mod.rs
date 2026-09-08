//! This module contains all of the filesystem scanner modules.
//!
//! The most common scanner to use is [`generic::Scanner`] which will
//! automatically detect and use the appropriate scanner.

use flix_model::id::{CollectionId, MovieId, ShowId};
use flix_model::numbers::{EpisodeNumbers, SeasonNumber};

pub mod library;

pub mod generic;

pub mod collection;

pub mod movie;

pub mod episode;
pub mod season;
pub mod show;

/// A reference to a piece of media.
#[expect(clippy::exhaustive_enums, reason = "unlikely to change")]
#[derive(Debug, Clone)]
pub enum MediaRef<ID> {
	/// An explicit ID.
	Id(ID),
	/// A filesystem slug.
	Slug(String),
}

impl<ID> MediaRef<ID> {
	/// Get the slug if it exists.
	#[inline]
	pub fn into_slug(self) -> Option<String> {
		match self {
			Self::Id(_) => None,
			Self::Slug(slug) => Some(slug),
		}
	}
}

/// A scanned collection.
#[derive(Debug)]
#[expect(
	clippy::exhaustive_structs,
	reason = "it will be a breaking change when more information is scanned"
)]
pub struct CollectionScan {
	/// The ID of the parent collection (if any).
	pub parent_ref: Option<MediaRef<CollectionId>>,
	/// The ID of the collection.
	pub id_ref: MediaRef<CollectionId>,
	/// The file name of the poster file.
	pub poster_file_name: Option<String>,
}

/// A scanned movie.
#[derive(Debug)]
#[expect(
	clippy::exhaustive_structs,
	reason = "it will be a breaking change when more information is scanned"
)]
pub struct MovieScan {
	/// The ID of the parent collection (if any).
	pub parent_ref: Option<MediaRef<CollectionId>>,
	/// The ID of the movie.
	pub id_ref: MediaRef<MovieId>,
	/// The file name of the media file.
	pub media_file_name: String,
	/// The file name of the poster file.
	pub poster_file_name: Option<String>,
}

/// A scanned show.
#[derive(Debug)]
#[expect(
	clippy::exhaustive_structs,
	reason = "it will be a breaking change when more information is scanned"
)]
pub struct ShowScan {
	/// The ID of the parent collection (if any).
	pub parent_ref: Option<MediaRef<CollectionId>>,
	/// The ID of the show.
	pub id_ref: MediaRef<ShowId>,
	/// The file name of the poster file.
	pub poster_file_name: Option<String>,
}

/// A scanned season.
#[derive(Debug)]
#[expect(
	clippy::exhaustive_structs,
	reason = "it will be a breaking change when more information is scanned"
)]
pub struct SeasonScan {
	/// The ID of the show this season belongs to.
	pub show_ref: MediaRef<ShowId>,
	/// The season this episode belongs to.
	pub season: SeasonNumber,
	/// The file name of the poster file.
	pub poster_file_name: Option<String>,
}

/// A scanned episode.
#[derive(Debug)]
#[expect(
	clippy::exhaustive_structs,
	reason = "it will be a breaking change when more information is scanned"
)]
pub struct EpisodeScan {
	/// The ID of the show this episode belongs to.
	pub show_ref: MediaRef<ShowId>,
	/// The season this episode belongs to.
	pub season: SeasonNumber,
	/// The number(s) of this episode.
	pub episode: EpisodeNumbers,
	/// The file name of the media file.
	pub media_file_name: String,
	/// The file name of the poster file.
	pub poster_file_name: Option<String>,
}

#[cfg(test)]
mod tests {
	use super::{MediaRef, MovieId, library};

	#[cfg(not(miri))]
	#[test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	fn use_fn() {
		let tmp = tempfile::tempdir().expect("temporary directory exists");
		drop(library::Scanner::scan_library(tmp.path()));
		drop(MediaRef::Id(MovieId::from_raw(0)).into_slug());
	}
}
