//! Collections API

use std::rc::Rc;
use std::sync::RwLock;

use crate::api::exec_request;
use crate::model::Collection;
use crate::model::id::CollectionId;
use crate::{Cache, CachePolicy, Config};

use super::{Error, make_request};

/// TMDB Collections API client
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
	/// Fetch the details of the collection refered to by ID
	pub async fn get_details(
		&self,
		id: impl Into<CollectionId>,
		language: Option<&str>,
	) -> Result<Collection, Error> {
		let request = make_request(
			&self.config,
			&format!("/3/collection/{}", id.into().into_raw()),
			language,
		)?;
		exec_request(&self.config, &*self.cache, &self.policy, request).await
	}
}
