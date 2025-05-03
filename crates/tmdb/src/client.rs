use std::rc::Rc;

use crate::{Config, api};

/// The primary client that references all other clients
pub struct Client {
	genres: api::genres::Client,
	collections: api::collections::Client,
	movies: api::movies::Client,
	shows: api::shows::Client,
	seasons: api::seasons::Client,
	episodes: api::episodes::Client,
}

impl Client {
	/// Create a new client from a default configuration using the bearer token
	pub fn new(bearer_token: String) -> Self {
		Self::new_with_config(Config::new(bearer_token))
	}

	/// Create a new client with the given configuration
	pub fn new_with_config(config: Config) -> Self {
		let config = Rc::new(config);
		Self {
			genres: api::genres::Client::new(config.clone()),
			collections: api::collections::Client::new(config.clone()),
			movies: api::movies::Client::new(config.clone()),
			shows: api::shows::Client::new(config.clone()),
			seasons: api::seasons::Client::new(config.clone()),
			episodes: api::episodes::Client::new(config.clone()),
		}
	}
}

impl Client {
	/// Access the Genres API
	pub fn genres(&self) -> &api::genres::Client {
		&self.genres
	}

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
