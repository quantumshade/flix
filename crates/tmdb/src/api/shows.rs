use std::rc::Rc;

use crate::Config;
use crate::model::{Show, ShowId};

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
		Ok(self
			.config
			.client
			.execute(make_request(
				&self.config,
				&format!("/3/tv/{}", id.into()),
				language,
			)?)
			.await?
			.error_for_status()?
			.json()
			.await?)
	}
}
