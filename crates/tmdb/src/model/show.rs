use chrono::NaiveDate;

use super::id::ShowId;

/// A deserialized Show from the TMDB API
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Show {
	/// The show's TMDB ID
	pub id: ShowId,
	/// The show's title
	#[serde(rename = "name")]
	pub title: String,
	/// The show's tagline
	pub tagline: String,
	/// The show's overview
	pub overview: String,
	/// The show's first air date
	pub first_air_date: NaiveDate,
	/// The show's last air date
	pub last_air_date: NaiveDate,
	/// The total number of episodes in this show
	pub number_of_episodes: u32,
	/// The number of seasons in this show
	pub number_of_seasons: u32,
}

// TODO: Genres
// pub genres: Vec<Genre>,
// where: struct Genre { id, name }

// TODO: Network
// pub networks: Vec<Network>
// where: struct Network { id, name }

// TODO: Company
// pub companies: Vec<Company>
// where: struct Company { id, name }
