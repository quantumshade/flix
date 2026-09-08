//! TMDB client.

use alloc::sync::Arc;
use std::sync::RwLock;

use crate::{Cache, CachePolicy, Config, api};

/// The primary client that references all other clients.
#[derive(Debug)]
pub struct Client {
	/// The collection API client.
	collections: api::collections::Client,
	/// The movie API client.
	movies: api::movies::Client,
	/// The show API client.
	shows: api::shows::Client,
	/// The season API client.
	seasons: api::seasons::Client,
	/// The episode API client.
	episodes: api::episodes::Client,

	/// The cache policy to use for all clients.
	cache_policy: Arc<RwLock<CachePolicy>>,
}

impl Client {
	/// Create a new client with the given configuration.
	#[inline]
	pub fn new(config: Config, cache: Arc<dyn Cache>, cache_policy: CachePolicy) -> Self {
		let config = Arc::new(config);
		let cache_policy = Arc::new(RwLock::new(cache_policy));
		Self {
			collections: api::collections::Client::new(
				Arc::clone(&config),
				Arc::clone(&cache),
				Arc::clone(&cache_policy),
			),
			movies: api::movies::Client::new(
				Arc::clone(&config),
				Arc::clone(&cache),
				Arc::clone(&cache_policy),
			),
			shows: api::shows::Client::new(
				Arc::clone(&config),
				Arc::clone(&cache),
				Arc::clone(&cache_policy),
			),
			seasons: api::seasons::Client::new(
				Arc::clone(&config),
				Arc::clone(&cache),
				Arc::clone(&cache_policy),
			),
			episodes: api::episodes::Client::new(config, cache, Arc::clone(&cache_policy)),

			cache_policy,
		}
	}

	/// Modify the [`CachePolicy`].
	#[inline]
	pub fn set_cache_policy(&self, new_policy: CachePolicy) {
		match self.cache_policy.write() {
			Ok(mut policy) => *policy = new_policy,
			Err(mut poison) => {
				**poison.get_mut() = new_policy;
				self.cache_policy.clear_poison();
			}
		}
	}

	/// Access the Collections API.
	#[inline]
	#[must_use]
	pub const fn collections(&self) -> &api::collections::Client {
		&self.collections
	}

	/// Access the Movies API.
	#[inline]
	#[must_use]
	pub const fn movies(&self) -> &api::movies::Client {
		&self.movies
	}

	/// Access the Shows API.
	#[inline]
	#[must_use]
	pub const fn shows(&self) -> &api::shows::Client {
		&self.shows
	}

	/// Access the Seasons API.
	#[inline]
	#[must_use]
	pub const fn seasons(&self) -> &api::seasons::Client {
		&self.seasons
	}

	/// Access the Episodes API.
	#[inline]
	#[must_use]
	pub const fn episodes(&self) -> &api::episodes::Client {
		&self.episodes
	}
}
