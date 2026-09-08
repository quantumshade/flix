//! Episodes API.

use alloc::sync::Arc;
use std::sync::RwLock;

use flix_model::numbers::{EpisodeNumber, SeasonNumber};

use crate::api::exec_request;
use crate::model::Episode;
use crate::model::id::ShowId;
use crate::{Cache, CachePolicy, Config};

use super::{Error, make_request};

/// TMDB Episodes API client.
#[derive(Debug)]
pub struct Client {
	/// The client configuration.
	config: Arc<Config>,
	/// The request cache.
	cache: Arc<dyn Cache>,
	/// The cache policy to use for requests.
	policy: Arc<RwLock<CachePolicy>>,
}

impl Client {
	/// Create a new client with the given configuration.
	#[inline]
	pub fn new(
		config: Arc<Config>,
		cache: Arc<dyn Cache>,
		policy: Arc<RwLock<CachePolicy>>,
	) -> Self {
		Self {
			config,
			cache,
			policy,
		}
	}

	/// Fetch the details of the episode refered to by ID, season number and episode number.
	///
	/// # Errors
	/// See [Error] for failure modes.
	#[inline]
	#[expect(clippy::impl_trait_in_params, reason = "turbofish is not expected")]
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
