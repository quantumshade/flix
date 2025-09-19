//! Deserializable types from the TMDB API

use core::time::Duration;

use serde::{Deserialize, Deserializer};

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
	let minutes = u64::deserialize(deserializer)?;
	Ok(Duration::from_secs(minutes.saturating_mul(60)))
}
