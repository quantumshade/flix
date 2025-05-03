use std::rc::Rc;

use crate::Config;
use crate::model::{Collection, CollectionId};

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
		Ok(self
			.config
			.client
			.execute(make_request(
				&self.config,
				&format!("/3/collection/{}", id.into()),
				language,
			)?)
			.await?
			.error_for_status()?
			.json()
			.await?)
	}
}
