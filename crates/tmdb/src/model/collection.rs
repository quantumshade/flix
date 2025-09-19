use super::id::{CollectionId, MovieId};

/// A deserialized Collection from the TMDB API
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Collection {
	/// The collection's TMDB ID
	pub id: CollectionId,
	/// The collection's title
	#[serde(rename = "name")]
	pub title: String,
	/// The collection's overview
	pub overview: String,
	/// The list of movies that are part of this collection
	#[serde(rename = "parts")]
	pub movies: Vec<Item>,
}

/// A deserialized collection item from the TMDB API
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Item {
	/// The movie's TMDB ID
	pub id: MovieId,
	/// The movie's title
	pub title: String,
}
