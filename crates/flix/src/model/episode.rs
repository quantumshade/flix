#[cfg(feature = "tmdb")]
use flix_tmdb::model::ShowId;

use chrono::NaiveDate;

/// An Episode container
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Episode {
	/// The generic episode data
	pub episode: GenericEpisode,

	/// The TMDB episode data
	#[cfg(feature = "tmdb")]
	pub tmdb: Option<TmdbEpisode>,
}

/// A wrapper for handling single and multi-episode entries
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EpisodeNumber {
	/// The entry contains a single episode
	Single {
		/// The episode's number
		number: u32,
	},
	/// The entry contains multiple episodes
	Multiple {
		/// The list of episode numbers
		numbers: Vec<u32>,
	},
}

impl EpisodeNumber {
	/// Get the primary episode number of this episode
	pub fn primary_episode_number(&self) -> Option<u32> {
		match self {
			EpisodeNumber::Single { number } => Some(*number),
			EpisodeNumber::Multiple { numbers } => numbers.first().copied(),
		}
	}

	/// Get additional episode numbers of this episode
	pub fn additional_episode_numbers(&self) -> Vec<u32> {
		match self {
			EpisodeNumber::Single { number: _ } => vec![],
			EpisodeNumber::Multiple { numbers } => numbers.iter().skip(1).copied().collect(),
		}
	}
}

/// The generic episode data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GenericEpisode {
	/// The episode's number(s)
	#[serde(flatten)]
	pub number: EpisodeNumber,
	/// The episode's season's number
	pub season: u32,
	/// The episode's title
	pub title: String,
	/// The episode's overview
	pub overview: String,
	/// The episode's air date
	pub air_date: NaiveDate,
}

/// The TMDB show data
#[cfg(feature = "tmdb")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TmdbEpisode {
	/// The episodes's show's TMDB ID
	pub show_id: ShowId,
}
