//! This module contains types relating to flix media IDs

use seamantic::model::id::Id;

/// Type alias for the raw ID representation
pub use seamantic::model::id::SeaOrmRepr as RawId;

#[doc(hidden)]
pub enum Library {}
/// Type alias for a library ID
pub type LibraryId = Id<Library>;

#[doc(hidden)]
pub enum Collection {}
/// Type alias for a collection ID
pub type CollectionId = Id<Collection>;

#[doc(hidden)]
pub enum Movie {}
/// Type alias for a movie ID
pub type MovieId = Id<Movie>;

#[doc(hidden)]
pub enum Show {}
/// Type alias for a show ID
pub type ShowId = Id<Show>;
