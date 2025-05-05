use chrono::NaiveDate;

use super::{ShowGenre, ShowId};

/// A deserialized Show from the TMDB API
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Show {
	/// The show's TMDB ID
	pub id: ShowId,
	/// The show's name
	pub name: String,
	/// The show's overview
	pub overview: String,
	/// The list of genres this show belongs to
	pub genres: Vec<ShowGenre>,
	/// The show's first air date
	pub first_air_date: NaiveDate,
	/// The show's last air date
	pub last_air_date: NaiveDate,
	/// The number of seasons in this show
	pub number_of_seasons: i32,
	/// The show's status
	pub status: ShowStatus,
}

/// A deserialized show Status from the TMDB API
#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub enum ShowStatus {
	/// The show is returning
	#[serde(rename = "Returning Series")]
	Returning,
	/// The show is planned
	#[serde(rename = "Planned")]
	Planned,
	/// The show is in procuction
	#[serde(rename = "In Production")]
	InProduction,
	/// The show has ended
	#[serde(rename = "Ended")]
	Ended,
	/// The show is canceled
	#[serde(rename = "Canceled")]
	Canceled,
	/// The show only released a pilot
	#[serde(rename = "Pilot")]
	Pilot,
}
