use core::time::Duration;

use flix_model::numbers::EpisodeNumber;

use chrono::NaiveDate;

use super::duration_from_minutes;

/// A deserialized Episode from the TMDB API
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Episode {
	/// The episode's number
	pub episode_number: EpisodeNumber,
	/// The episode's title
	#[serde(rename = "name")]
	pub title: String,
	/// The episode's overview
	pub overview: String,
	/// The episode's air date
	pub air_date: NaiveDate,
	/// The movie's runtime
	#[serde(deserialize_with = "duration_from_minutes")]
	pub runtime: Duration,
}
