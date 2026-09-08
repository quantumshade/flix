//! TMDB episode model.

use core::time::Duration;

use flix_model::numbers::EpisodeNumber;

use chrono::NaiveDate;
use url::Url;

use super::{duration_from_minutes, still_url_from_path};

/// A deserialized Episode from the TMDB API.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct Episode {
	/// The episode's number.
	pub episode_number: EpisodeNumber,
	/// The episode's title.
	#[serde(rename = "name")]
	pub title: String,
	/// The episode's overview.
	pub overview: String,
	/// The episode's air date.
	pub air_date: NaiveDate,
	/// The episode's runtime.
	#[serde(deserialize_with = "duration_from_minutes")]
	pub runtime: Duration,
	/// The episode's still path.
	#[serde(deserialize_with = "still_url_from_path")]
	pub still_path: Option<Url>,
}

#[cfg(test)]
mod tests {
	use core::time::Duration;

	use chrono::NaiveDate;

	use super::{Episode, EpisodeNumber};

	#[test]
	fn use_types() {
		drop(Episode {
			episode_number: EpisodeNumber::new(0),
			title: String::new(),
			overview: String::new(),
			air_date: NaiveDate::default(),
			runtime: Duration::default(),
			still_path: None,
		});
	}
}
