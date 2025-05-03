use super::id::{MovieGenreId, ShowGenreId};

/// A deserialized movie Genre from the TMDB API
#[derive(Debug, serde::Deserialize)]
pub struct MovieGenre {
	/// The genre's TMDB ID
	pub id: MovieGenreId,
	/// The genre's name
	pub name: String,
}

/// A deserialized show Genre from the TMDB API
#[derive(Debug, serde::Deserialize)]
pub struct ShowGenre {
	/// The genre's TMDB ID
	pub id: ShowGenreId,
	/// The genre's name
	pub name: String,
}
