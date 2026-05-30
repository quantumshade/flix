//! Deserializable types from the TMDB API

use core::str::FromStr;
use core::time::Duration;

use serde::{Deserialize, Deserializer};
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

fn duration_from_minutes<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
	D: Deserializer<'de>,
{
	let minutes = u64::deserialize(deserializer).unwrap_or(0);
	Ok(Duration::from_secs(minutes.saturating_mul(60)))
}

fn still_url_from_path<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
where
	D: Deserializer<'de>,
{
	const TMDB_IMAGE_BASE: &str = "http://image.tmdb.org/t/p/";
	const TMDB_IMAGE_QUALITY: &str = "original";

	let path = Option::<&str>::deserialize(deserializer)?;

	Ok(path.and_then(|path| {
		Url::from_str(&format!(
			"{}{}{}",
			TMDB_IMAGE_BASE, TMDB_IMAGE_QUALITY, path
		))
		.ok()
	}))
}
