use std::rc::Rc;

use crate::Config;
use crate::model::{Episode, ShowId};

use super::{Error, make_request};

/// TMDB Episodes API client
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
	/// Fetch the details of the episode refered to by ID, season number and episode number
	pub async fn get_details(
		&self,
		id: impl Into<ShowId>,
		season: impl Into<u32>,
		episode: impl Into<u32>,
		language: Option<&str>,
	) -> Result<Episode, Error> {
		Ok(self
			.config
			.client
			.execute(make_request(
				&self.config,
				&format!(
					"/3/tv/{}/season/{}/episode/{}",
					id.into(),
					season.into(),
					episode.into()
				),
				language,
			)?)
			.await?
			.error_for_status()?
			.json()
			.await?)
	}
}
