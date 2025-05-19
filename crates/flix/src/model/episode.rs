#[cfg(feature = "tmdb")]
use flix_tmdb::model::ShowId;

use chrono::NaiveDate;

/// An Episode container
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Episode {
	/// The generic episode data
	pub episode: GenericEpisode,

	/// The TMDB episode data
	#[cfg(feature = "tmdb")]
	pub tmdb: Option<TmdbEpisode>,
}

/// The generic episode data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GenericEpisode {
	/// The episode's number
	pub number: u32,
	/// The episode's season's number
	pub season: u32,
	/// The episode's title
	pub title: String,
	/// The episode's overview
	pub overview: String,
	/// The episode's air date
	pub air_date: NaiveDate,
}

/// The TMDB show data
#[cfg(feature = "tmdb")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TmdbEpisode {
	/// The episodes's show's TMDB ID
	pub show_id: ShowId,
}
