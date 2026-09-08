//! TMDB API clients.

use core::time::Duration;
use std::sync::RwLock;

use governor::Jitter;
use reqwest::Request;
use reqwest::header;
use serde::de::DeserializeOwned;

use crate::{Cache, CachePolicy, Config};

pub mod collections;
pub mod episodes;
pub mod movies;
pub mod seasons;
pub mod shows;

/// A generic error wrapping Url and Reqwest errors.
#[derive(Debug, thiserror::Error)]
#[expect(clippy::error_impl_error, reason = "Error is a good name here")]
#[expect(clippy::exhaustive_enums, reason = "unlikely to add new variants")]
pub enum Error {
	/// Url error wrapper.
	#[error("url parse error: {0}")]
	Url(#[from] url::ParseError),
	/// Reqwest error wrapper.
	#[error("reqwest error: {0}")]
	Reqwest(#[from] reqwest::Error),
	/// Json error wrapper.
	#[error("json error: {0}")]
	Json(#[from] serde_json::Error),
}

/// Turn a [Config] and `path` into a [Request].
///
/// # Errors
/// See [Error] for failure modes.
fn make_request(config: &Config, path: &str, language: Option<&str>) -> Result<Request, Error> {
	let url = config.base_url.join(path)?;

	let mut builder = config.client.get(url).header(
		header::AUTHORIZATION,
		format!("Bearer {}", config.bearer_token),
	);
	if let Some(user_agent) = &config.user_agent {
		builder = builder.header(header::USER_AGENT, user_agent);
	}
	if let Some(language) = language {
		builder = builder.query(&[("language", language)]);
	}

	Ok(builder.build()?)
}

/// Execute a [Request] and deserialize the response.
///
/// # Errors
/// See [Error] for failure modes.
async fn exec_request<T: DeserializeOwned>(
	config: &Config,
	cache: &dyn Cache,
	policy: &RwLock<CachePolicy>,
	request: Request,
) -> Result<T, Error> {
	let (read_cache, write_cache) = policy.read().map_or((None, None), |guard| match *guard {
		CachePolicy::None => (None, None),
		CachePolicy::Full => (Some(cache), Some(cache)),
		CachePolicy::Read => (Some(cache), None),
		CachePolicy::Update => (None, Some(cache)),
	});

	let path = request.url().path().to_owned();

	// read the cache and fall back to reqwest
	let mut response = None;
	if let Some(cache) = read_cache {
		response = cache.get(&path);
	}
	let needs_cache_write = response.is_none();
	let response = if let Some(response) = response {
		response
	} else {
		config
			.limiter
			.until_ready_with_jitter(Jitter::new(
				Duration::from_millis(0),
				Duration::from_millis(50),
			))
			.await;
		config
			.client
			.execute(request)
			.await?
			.error_for_status()?
			.bytes()
			.await?
	};

	// write to the cache if needed
	if let Some(cache) = write_cache
		&& needs_cache_write
	{
		cache.set(&path, &response);
	}

	Ok(serde_json::from_slice(&response)?)
}

#[cfg(test)]
mod tests {
	use super::Error;

	#[test]
	fn use_types() {
		drop(Error::Url(url::ParseError::Overflow));
	}
}
