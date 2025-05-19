#[cfg(feature = "tmdb")]
use flix_tmdb::model::{ShowGenreId, ShowId};

use chrono::NaiveDate;

/// A Show container
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Show {
	/// The generic show data
	pub show: GenericShow,

	/// The TMDB show data
	#[cfg(feature = "tmdb")]
	pub tmdb: Option<TmdbShow>,
}

/// The generic show data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GenericShow {
	/// The show's title
	pub title: String,
	/// The show's overview
	pub overview: String,
	/// The show's genres
	pub genres: Vec<String>,
	/// The show's air date
	pub air_date: NaiveDate,
}

/// The TMDB show data
#[cfg(feature = "tmdb")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TmdbShow {
	/// The show's TMDB ID
	pub id: ShowId,
	/// The list of genre TMDB IDs that the movie is associated with
	pub genres: Vec<ShowGenreId>,
}
