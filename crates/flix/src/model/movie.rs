#[cfg(feature = "tmdb")]
use flix_tmdb::model::{MovieGenreId, MovieId};

use chrono::NaiveDate;

/// A Movie container
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Movie {
	/// The generic movie data
	pub movie: GenericMovie,

	/// The TMDB movie data
	#[cfg(feature = "tmdb")]
	pub tmdb: Option<TmdbMovie>,
}

/// The generic movie data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GenericMovie {
	/// The movie's title
	pub title: String,
	/// The movie's overview
	pub overview: String,
	/// The movie's genres
	pub genres: Vec<String>,
	/// The movie's release date
	pub release_date: NaiveDate,
}

/// The TMDB movie data
#[cfg(feature = "tmdb")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TmdbMovie {
	/// The movie's TMDB ID
	pub id: MovieId,
	/// The list of genre TMDB IDs that the movie is associated with
	pub genres: Vec<MovieGenreId>,
}
