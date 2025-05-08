use chrono::NaiveDate;

/// A deserialized Episode from the TMDB API
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Episode {
	/// The episode's number
	pub episode_number: u32,
	/// The episode's name
	pub name: String,
	/// The episode's overview
	pub overview: String,
	/// The episode's air date
	pub air_date: NaiveDate,
}
