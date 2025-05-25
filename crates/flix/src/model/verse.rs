#[cfg(feature = "tmdb")]
use flix_tmdb::model::{MovieId, ShowId};

/// A Verse container
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Verse {
	/// The generic verse data
	pub verse: GenericVerse,

	/// The TMDB verse data
	#[cfg(feature = "tmdb")]
	pub tmdb: Option<TmdbVerse>,
}

/// The generic verse data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GenericVerse {
	/// The verse's title
	pub title: String,
	/// The verse's overview
	pub overview: String,
}

/// The TMDB verse data
#[cfg(feature = "tmdb")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TmdbVerse {
	/// The list of movie TMDB IDs in the verse
	pub movies: Vec<MovieId>,
	/// The list of show TMDB IDs in the verse
	pub shows: Vec<ShowId>,
	/// The list of sub-verse names
	pub verses: Vec<String>,
}
