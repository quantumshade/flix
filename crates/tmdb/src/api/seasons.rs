//! Seasons API

use std::rc::Rc;
use std::sync::RwLock;

use flix_model::numbers::SeasonNumber;

use crate::api::exec_request;
use crate::model::Season;
use crate::model::id::ShowId;
use crate::{Cache, CachePolicy, Config};

use super::{Error, make_request};

/// TMDB Seasons API client
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
	/// Fetch the details of the season refered to by ID and season number
	pub async fn get_details(
		&self,
		id: impl Into<ShowId>,
		season: impl Into<SeasonNumber>,
		language: Option<&str>,
	) -> Result<Season, Error> {
		let request = make_request(
			&self.config,
			&format!("/3/tv/{}/season/{}", id.into().into_raw(), season.into()),
			language,
		)?;
		exec_request(&self.config, &*self.cache, &self.policy, request).await
	}
}
