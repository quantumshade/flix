use std::rc::Rc;

use crate::Config;
use crate::model::{Season, ShowId};

use super::{Error, make_request};

/// TMDB Seasons API client
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
	/// Fetch the details of the season refered to by ID and season number
	pub async fn get_details(
		&self,
		id: impl Into<ShowId>,
		season: impl Into<i32>,
		language: Option<&str>,
	) -> Result<Season, Error> {
		Ok(self
			.config
			.client
			.execute(make_request(
				&self.config,
				&format!("/3/tv/{}/season/{}", id.into(), season.into()),
				language,
			)?)
			.await?
			.error_for_status()?
			.json()
			.await?)
	}
}
