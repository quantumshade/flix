use std::rc::Rc;

use crate::Config;
use crate::model::{MovieGenre, ShowGenre};

use super::{Error, make_request};

/// TMDB Genre API client
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
	/// Fetch the list of all valid movie genres
	pub async fn get_movie_genres(&self, language: Option<&str>) -> Result<Vec<MovieGenre>, Error> {
		#[derive(Debug, serde::Deserialize)]
		struct Genres {
			genres: Vec<MovieGenre>,
		}

		let genres: Genres = self
			.config
			.client
			.execute(make_request(&self.config, "/3/genre/movie/list", language)?)
			.await?
			.error_for_status()?
			.json()
			.await?;

		Ok(genres.genres)
	}

	/// Fetch the list of all valid show genres
	pub async fn get_tv_genres(&self, language: Option<&str>) -> Result<Vec<ShowGenre>, Error> {
		#[derive(Debug, serde::Deserialize)]
		struct Genres {
			genres: Vec<ShowGenre>,
		}

		let genres: Genres = self
			.config
			.client
			.execute(make_request(&self.config, "/3/genre/tv/list", language)?)
			.await?
			.error_for_status()?
			.json()
			.await?;

		Ok(genres.genres)
	}
}
