//! Movies API

use std::rc::Rc;
use std::sync::RwLock;

use crate::api::exec_request;
use crate::model::Movie;
use crate::model::id::MovieId;
use crate::{Cache, CachePolicy, Config};

use super::{Error, make_request};

/// TMDB Movies API client
pub struct Client {
	config: Rc<Config>,
	cache: Rc<dyn Cache>,
	policy: Rc<RwLock<CachePolicy>>,
}

impl Client {
	/// Create a new client with the given configuration
	pub fn new(config: Rc<Config>, cache: Rc<dyn Cache>, policy: Rc<RwLock<CachePolicy>>) -> Self {
		Self {
			config,
			cache,
			policy,
		}
	}
}

impl Client {
	/// Fetch the details of the movie refered to by ID
	pub async fn get_details(
		&self,
		id: impl Into<MovieId>,
		language: Option<&str>,
	) -> Result<Movie, Error> {
		let request = make_request(
			&self.config,
			&format!("/3/movie/{}", id.into().into_raw()),
			language,
		)?;
		exec_request(&self.config, &*self.cache, &self.policy, request).await
	}
}
