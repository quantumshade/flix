//! Episodes API

use std::rc::Rc;
use std::sync::RwLock;

use flix_model::numbers::{EpisodeNumber, SeasonNumber};

use crate::api::exec_request;
use crate::model::Episode;
use crate::model::id::ShowId;
use crate::{Cache, CachePolicy, Config};

use super::{Error, make_request};

/// TMDB Episodes API client
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
	/// Fetch the details of the episode refered to by ID, season number and episode number
	pub async fn get_details(
		&self,
		id: impl Into<ShowId>,
		season: impl Into<SeasonNumber>,
		episode: impl Into<EpisodeNumber>,
		language: Option<&str>,
	) -> Result<Episode, Error> {
		let request = make_request(
			&self.config,
			&format!(
				"/3/tv/{}/season/{}/episode/{}",
				id.into().into_raw(),
				season.into(),
				episode.into()
			),
			language,
		)?;
		exec_request(&self.config, &*self.cache, &self.policy, request).await
	}
}
