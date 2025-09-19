use core::time::Duration;

use chrono::NaiveDate;

use super::duration_from_minutes;
use super::id::{CollectionId, MovieId};

/// A deserialized Movie from the TMDB API
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Movie {
	/// The movie's TMDB ID
	pub id: MovieId,
	/// The movie's collection, if it exists
	#[serde(rename = "belongs_to_collection")]
	pub collection: Option<InCollection>,
	/// The movie's title
	pub title: String,
	/// The movie's tagline
	pub tagline: String,
	/// The movie's overview
	pub overview: String,
	/// The movie's release date
	pub release_date: NaiveDate,
	/// The movie's runtime
	#[serde(deserialize_with = "duration_from_minutes")]
	pub runtime: Duration,
}

/// A deserialized movie's collection from the TMDB API
#[derive(Debug, Clone, serde::Deserialize)]
pub struct InCollection {
	/// The collection's TMDB ID
	pub id: CollectionId,
	/// The collection's title
	#[serde(rename = "name")]
	pub title: String,
}

// TODO: Genres
// pub genres: Vec<Genre>,
// where: struct Genre { id, name }

// TODO: Company
// pub companies: Vec<Company>
// where: struct Company { id, name }
