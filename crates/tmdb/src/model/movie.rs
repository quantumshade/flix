use chrono::NaiveDate;

use super::{CollectionId, MovieGenre, MovieId};

/// A deserialized Movie from the TMDB API
#[derive(Debug, serde::Deserialize)]
pub struct Movie {
	/// The movie's TMDB ID
	pub id: MovieId,
	/// The movie's collection, if it exists
	#[serde(rename = "belongs_to_collection")]
	pub collection: Option<InCollection>,
	/// The movie's title
	pub title: String,
	/// The movie's overview
	pub overview: String,
	/// The list of genres the movie belongs to
	pub genres: Vec<MovieGenre>,
	/// The movie's release date
	pub release_date: NaiveDate,
	/// The movie's status
	pub status: MovieStatus,
}

/// A deserialized movie's collection from the TMDB API
#[derive(Debug, serde::Deserialize)]
pub struct InCollection {
	/// The collection's TMDB ID
	pub id: CollectionId,
}

/// A deserialized movie status from the TMDB API
#[derive(Debug, serde::Deserialize)]
pub enum MovieStatus {
	/// The movie was cancelled
	#[serde(rename = "Canceled")]
	Canceled,
	/// The movie is in production
	#[serde(rename = "In Production")]
	InProduction,
	/// The movie is planned
	#[serde(rename = "Planned")]
	Planned,
	/// The movie is in post production
	#[serde(rename = "Post Production")]
	PostProduction,
	/// The movie is released
	#[serde(rename = "Released")]
	Released,
	/// The movie is rumored
	#[serde(rename = "Rumored")]
	Rumored,
}
