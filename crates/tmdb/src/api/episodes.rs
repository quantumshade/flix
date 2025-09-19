//! Episodes API

use core::time::Duration;
use std::rc::Rc;

use flix_model::numbers::{EpisodeNumber, SeasonNumber};

use governor::Jitter;

use crate::Config;
use crate::model::Episode;
use crate::model::id::ShowId;

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
		season: impl Into<SeasonNumber>,
		episode: impl Into<EpisodeNumber>,
		language: Option<&str>,
	) -> Result<Episode, Error> {
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
				&format!(
					"/3/tv/{}/season/{}/episode/{}",
					id.into().into_raw(),
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
