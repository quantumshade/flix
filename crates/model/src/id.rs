//! This module contains types relating to flix media IDs.

#![expect(clippy::module_name_repetitions, reason = "ID types end with Id")]

use seamantic::model::id::Id;

/// Type alias for the raw ID representation.
pub use seamantic::model::id::SeaOrmRepr as RawId;

#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Library {}
/// Type alias for a library ID.
pub type LibraryId = Id<Library>;

#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Collection {}
/// Type alias for a collection ID.
pub type CollectionId = Id<Collection>;

#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Movie {}
/// Type alias for a movie ID.
pub type MovieId = Id<Movie>;

#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Show {}
/// Type alias for a show ID.
pub type ShowId = Id<Show>;

#[cfg(test)]
mod tests {
	use super::{CollectionId, LibraryId, MovieId, ShowId};

	#[test]
	fn use_types() {
		_ = CollectionId::from_raw(0);
		_ = LibraryId::from_raw(0);
		_ = MovieId::from_raw(0);
		_ = ShowId::from_raw(0);
	}
}
