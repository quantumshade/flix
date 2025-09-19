use chrono::NaiveDate;

use flix_model::numbers::SeasonNumber;

/// A deserialized Season from the TMDB API
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Season {
	/// The season's number
	pub season_number: SeasonNumber,
	/// The season's title
	#[serde(rename = "name")]
	pub title: String,
	/// The season's overview
	pub overview: String,
	/// The season's air date
	pub air_date: NaiveDate,
	/// The number of episodes in this season
	pub episodes: Vec<FakeEpisode>,
}

/// A placeholder struct for parsing the episodes list for a season
#[derive(Debug, Clone, serde::Deserialize)]
pub struct FakeEpisode {}
