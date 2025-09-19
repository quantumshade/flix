//! Shows API

use core::time::Duration;
use std::rc::Rc;

use governor::Jitter;

use crate::Config;
use crate::model::Show;
use crate::model::id::ShowId;

use super::{Error, make_request};

/// TMDB Shows API client
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
	/// Fetch the details of the show refered to by ID
	pub async fn get_details(
		&self,
		id: impl Into<ShowId>,
		language: Option<&str>,
	) -> Result<Show, Error> {
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
				&format!("/3/tv/{}", id.into().into_raw()),
				language,
			)?)
			.await?
			.error_for_status()?
			.json()
			.await?)
	}
}
