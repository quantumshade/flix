#[cfg(feature = "tmdb")]
use flix_tmdb::model::CollectionId;

/// A Collection container
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Collection {
	/// Generic collection data
	pub collection: GenericCollection,

	/// TMDB collection data
	#[cfg(feature = "tmdb")]
	pub tmdb: Option<TmdbCollection>,
}

/// The generic collection data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GenericCollection {
	/// The collection's title
	pub title: String,
	/// The collection's overview
	pub overview: String,
}

/// The TMDB collection data
#[cfg(feature = "tmdb")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TmdbCollection {
	/// The collection's TMDB ID
	pub id: CollectionId,
}
