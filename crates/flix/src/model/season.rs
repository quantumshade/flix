#[cfg(feature = "tmdb")]
use flix_tmdb::model::ShowId;

use chrono::NaiveDate;

/// A Season container
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Season {
	/// The generic season data
	pub season: GenericSeason,

	/// The TMDB season data
	#[cfg(feature = "tmdb")]
	pub tmdb: Option<TmdbSeason>,
}

/// The generic season data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GenericSeason {
	/// The season's number
	pub number: u32,
	/// The season's title
	pub title: String,
	/// The season's overview
	pub overview: String,
	/// The season's air date
	pub air_date: NaiveDate,
}

/// The TMDB show data
#[cfg(feature = "tmdb")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TmdbSeason {
	/// The season's show's TMDB ID
	pub show_id: ShowId,
}
