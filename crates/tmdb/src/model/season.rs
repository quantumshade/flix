//! TMDB season model.

use chrono::NaiveDate;

use flix_model::numbers::SeasonNumber;

/// A deserialized Season from the TMDB API.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct Season {
	/// The season's number.
	pub season_number: SeasonNumber,
	/// The season's title.
	#[serde(rename = "name")]
	pub title: String,
	/// The season's overview.
	pub overview: String,
	/// The season's air date.
	pub air_date: NaiveDate,
	/// The number of episodes in this season.
	pub episodes: Vec<FakeEpisode>,
}

/// A placeholder struct for parsing the episodes list for a season.
#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[non_exhaustive]
#[expect(clippy::empty_structs_with_brackets, reason = "might add fields later")]
pub struct FakeEpisode {}

#[cfg(test)]
mod tests {
	use chrono::NaiveDate;

	use super::{Season, SeasonNumber};

	#[test]
	fn use_types() {
		drop(Season {
			season_number: SeasonNumber::new(0),
			title: String::new(),
			overview: String::new(),
			air_date: NaiveDate::default(),
			episodes: Vec::new(),
		});
	}
}
