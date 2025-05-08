use chrono::NaiveDate;

/// An Episode container
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Episode {
	/// The generic episode data
	pub episode: GenericEpisode,
}

/// The generic episode data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GenericEpisode {
	/// The episode's number
	pub number: u32,
	/// The episode's name
	pub name: String,
	/// The episode's overview
	pub overview: String,
	/// The episode's air date
	pub air_date: NaiveDate,
}
