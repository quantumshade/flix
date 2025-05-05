use chrono::NaiveDate;

use super::Episode;

/// A deserialized Season from the TMDB API
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Season {
	/// The season's number
	pub season_number: i32,
	/// The season's name
	pub name: String,
	/// The season's overview
	pub overview: String,
	/// The season's air date
	pub air_date: NaiveDate,
	/// The list of episodes in this season
	pub episodes: Vec<Episode>,
}
