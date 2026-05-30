//! This module contains all of the filesystem scanner modules
//!
//! The most common scanner to use is [generic::Scanner] which will
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

/// A reference to a piece of media
#[derive(Debug, Clone)]
pub enum MediaRef<ID> {
	/// An explicit ID
	Id(ID),
	/// A filesystem slug
	Slug(String),
}

impl<ID> MediaRef<ID> {
	/// Get the slug if it exists
	pub fn into_slug(self) -> Option<String> {
		match self {
			MediaRef::Id(_) => None,
			MediaRef::Slug(slug) => Some(slug),
		}
	}
}

/// A scanned collection
#[derive(Debug)]
pub struct CollectionScan {
	/// The ID of the parent collection (if any)
	pub parent_ref: Option<MediaRef<CollectionId>>,
	/// The ID of the collection
	pub id_ref: MediaRef<CollectionId>,
	/// The file name of the poster file
	pub poster_file_name: Option<String>,
}

/// A scanned movie
#[derive(Debug)]
pub struct MovieScan {
	/// The ID of the parent collection (if any)
	pub parent_ref: Option<MediaRef<CollectionId>>,
	/// The ID of the movie
	pub id_ref: MediaRef<MovieId>,
	/// The file name of the media file
	pub media_file_name: String,
	/// The file name of the poster file
	pub poster_file_name: Option<String>,
}

/// A scanned show
#[derive(Debug)]
pub struct ShowScan {
	/// The ID of the parent collection (if any)
	pub parent_ref: Option<MediaRef<CollectionId>>,
	/// The ID of the show
	pub id_ref: MediaRef<ShowId>,
	/// The file name of the poster file
	pub poster_file_name: Option<String>,
}

/// A scanned season
#[derive(Debug)]
pub struct SeasonScan {
	/// The ID of the show this season belongs to
	pub show_ref: MediaRef<ShowId>,
	/// The season this episode belongs to
	pub season: SeasonNumber,
	/// The file name of the poster file
	pub poster_file_name: Option<String>,
}

/// A scanned episode
#[derive(Debug)]
pub struct EpisodeScan {
	/// The ID of the show this episode belongs to
	pub show_ref: MediaRef<ShowId>,
	/// The season this episode belongs to
	pub season: SeasonNumber,
	/// The number(s) of this episode
	pub episode: EpisodeNumbers,
	/// The file name of the media file
	pub media_file_name: String,
	/// The file name of the poster file
	pub poster_file_name: Option<String>,
}
