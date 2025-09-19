//! Collections API

use core::time::Duration;
use std::rc::Rc;

use governor::Jitter;

use crate::Config;
use crate::model::Collection;
use crate::model::id::CollectionId;

use super::{Error, make_request};

/// TMDB Collections API client
pub struct Client {
	config: Rc<Config>,
}

impl Client {
	/// Create a new client with the given configuration
	pub fn new(config: Rc<Config>) -> Self {
		Self { config }
	}
}

impl Client {
	/// Fetch the details of the collection refered to by ID
	pub async fn get_details(
		&self,
		id: impl Into<CollectionId>,
		language: Option<&str>,
	) -> Result<Collection, Error> {
		self.config
			.limiter
			.until_ready_with_jitter(Jitter::new(
				Duration::from_millis(0),
				Duration::from_millis(50),
			))
			.await;

		Ok(self
			.config
			.client
			.execute(make_request(
				&self.config,
				&format!("/3/collection/{}", id.into().into_raw()),
				language,
			)?)
			.await?
			.error_for_status()?
			.json()
			.await?)
	}
}
