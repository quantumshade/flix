use std::rc::Rc;

use crate::Config;
use crate::model::{Movie, MovieId};

use super::{Error, make_request};

/// TMDB Movies API client
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
	/// Fetch the details of the movie refered to by ID
	pub async fn get_details(
		&self,
		id: impl Into<MovieId>,
		language: Option<&str>,
	) -> Result<Movie, Error> {
		Ok(self
			.config
			.client
			.execute(make_request(
				&self.config,
				&format!("/3/movie/{}", id.into()),
				language,
			)?)
			.await?
			.error_for_status()?
			.json()
			.await?)
	}
}
