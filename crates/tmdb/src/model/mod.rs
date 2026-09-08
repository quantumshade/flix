//! Deserializable types from the TMDB API.

use core::str::FromStr as _;
use core::time::Duration;

use serde::{Deserialize as _, Deserializer};
use url::Url;

pub mod id;

mod collection;
mod episode;
mod movie;
mod season;
mod show;

pub use collection::*;
pub use episode::*;
pub use movie::*;
pub use season::*;
pub use show::*;

/// Deserializer for converting integer minutes to a [Duration].
///
/// # Errors
/// Fails if the value being deserialized isn't a [u64].
fn duration_from_minutes<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
	D: Deserializer<'de>,
{
	let minutes = u64::deserialize(deserializer)?;
	Ok(Duration::from_secs(minutes.saturating_mul(60)))
}

/// Deserializer for converting a string to a [Url].
///
/// # Errors
/// Fails if the value being deserialized isn't a [`&str`].
fn still_url_from_path<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
where
	D: Deserializer<'de>,
{
	const TMDB_IMAGE_BASE: &str = "http://image.tmdb.org/t/p/";
	const TMDB_IMAGE_QUALITY: &str = "original";

	let path = Option::<&str>::deserialize(deserializer)?;

	Ok(path.and_then(|path| {
		Url::from_str(&format!("{TMDB_IMAGE_BASE}{TMDB_IMAGE_QUALITY}{path}")).ok()
	}))
}
