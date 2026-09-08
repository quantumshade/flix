//! TMDB collection model.

use super::id::{CollectionId, MovieId};

/// A deserialized Collection from the TMDB API.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct Collection {
	/// The collection's TMDB ID.
	pub id: CollectionId,
	/// The collection's title.
	#[serde(rename = "name")]
	pub title: String,
	/// The collection's overview.
	pub overview: String,
	/// The list of movies that are part of this collection.
	#[serde(rename = "parts")]
	pub movies: Vec<Item>,
}

/// A deserialized collection item from the TMDB API.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct Item {
	/// The movie's TMDB ID.
	pub id: MovieId,
	/// The movie's title.
	pub title: String,
}

#[cfg(test)]
mod tests {
	use super::{Collection, CollectionId, Item, MovieId};

	#[test]
	fn use_types() {
		drop(Collection {
			id: CollectionId::from_raw(0),
			title: String::new(),
			overview: String::new(),
			movies: Vec::new(),
		});
		drop(Item {
			id: MovieId::from_raw(0),
			title: String::new(),
		});
	}
}
