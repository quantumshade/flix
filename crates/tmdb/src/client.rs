use std::rc::Rc;
use std::sync::RwLock;

use crate::{Cache, CachePolicy, Config, api};

/// The primary client that references all other clients
pub struct Client {
	collections: api::collections::Client,
	movies: api::movies::Client,
	shows: api::shows::Client,
	seasons: api::seasons::Client,
	episodes: api::episodes::Client,

	cache_policy: Rc<RwLock<CachePolicy>>,
}

impl Client {
	/// Create a new client with the given configuration
	pub fn new(config: Config, cache: Rc<dyn Cache>, cache_policy: CachePolicy) -> Self {
		let config = Rc::new(config);
		let cache_policy = Rc::new(RwLock::new(cache_policy));
		Self {
			collections: api::collections::Client::new(
				config.clone(),
				cache.clone(),
				cache_policy.clone(),
			),
			movies: api::movies::Client::new(config.clone(), cache.clone(), cache_policy.clone()),
			shows: api::shows::Client::new(config.clone(), cache.clone(), cache_policy.clone()),
			seasons: api::seasons::Client::new(config.clone(), cache.clone(), cache_policy.clone()),
			episodes: api::episodes::Client::new(
				config.clone(),
				cache.clone(),
				cache_policy.clone(),
			),

			cache_policy,
		}
	}

	/// Modify the [CachePolicy]
	pub fn set_cache_policy(&self, new_policy: CachePolicy) {
		match self.cache_policy.write() {
			Ok(mut policy) => *policy = new_policy,
			Err(mut poison) => {
				**poison.get_mut() = new_policy;
				self.cache_policy.clear_poison();
			}
		}
	}
}

impl Client {
	/// Access the Collections API
	pub fn collections(&self) -> &api::collections::Client {
		&self.collections
	}

	/// Access the Movies API
	pub fn movies(&self) -> &api::movies::Client {
		&self.movies
	}

	/// Access the Shows API
	pub fn shows(&self) -> &api::shows::Client {
		&self.shows
	}

	/// Access the Seasons API
	pub fn seasons(&self) -> &api::seasons::Client {
		&self.seasons
	}

	/// Access the Episodes API
	pub fn episodes(&self) -> &api::episodes::Client {
		&self.episodes
	}
}
