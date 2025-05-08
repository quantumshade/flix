use chrono::NaiveDate;

/// A Season container
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Season {
	/// The generic season data
	pub season: GenericSeason,
}

/// The generic season data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GenericSeason {
	/// The season's number
	pub number: u32,
	/// The season's name
	pub name: String,
	/// The season's overview
	pub overview: String,
	/// The season's air date
	pub air_date: NaiveDate,
}
