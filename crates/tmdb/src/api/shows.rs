//! Shows API.

use alloc::sync::Arc;
use std::sync::RwLock;

use crate::api::exec_request;
use crate::model::Show;
use crate::model::id::ShowId;
use crate::{Cache, CachePolicy, Config};

use super::{Error, make_request};

/// TMDB Shows API client.
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

	/// Fetch the details of the show refered to by ID.
	///
	/// # Errors
	/// See [Error] for failure modes.
	#[inline]
	#[expect(clippy::impl_trait_in_params, reason = "turbofish is not expected")]
	pub async fn get_details(
		&self,
		id: impl Into<ShowId>,
		language: Option<&str>,
	) -> Result<Show, Error> {
		let request = make_request(
			&self.config,
			&format!("/3/tv/{}", id.into().into_raw()),
			language,
		)?;
		exec_request(&self.config, &*self.cache, &self.policy, request).await
	}
}
