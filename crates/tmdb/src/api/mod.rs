use reqwest::Request;
use reqwest::header;

use crate::Config;

/// Collections API
pub mod collections;
/// Episodes API
pub mod episodes;
/// Genres API
pub mod genres;
/// Movies API
pub mod movies;
/// Seasons API
pub mod seasons;
/// Shows API
pub mod shows;

/// A generic error wrapping Url and Reqwest errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Url error wrapper
	#[error("url parse error: {0}")]
	Url(#[from] url::ParseError),
	/// Reqwest error wrapper
	#[error("reqwest error: {0}")]
	Reqwest(#[from] reqwest::Error),
}

fn make_request(config: &Config, path: &str, language: Option<&str>) -> Result<Request, Error> {
	let url = config.base_url.join(path)?;

	let mut builder = config.client.get(url).header(
		header::AUTHORIZATION,
		format!("Bearer {}", config.bearer_token),
	);
	if let Some(ref user_agent) = config.user_agent {
		builder = builder.header(header::USER_AGENT, user_agent);
	}
	if let Some(language) = language {
		builder = builder.query(&[("language", language)]);
	}

	Ok(builder.build()?)
}
