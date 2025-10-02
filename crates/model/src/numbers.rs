//! This module contains season and episode numbers and related errors

use core::ops::RangeInclusive;
use std::collections::HashSet;

/// Type alias for representing season numbers
pub type SeasonNumber = u32;
/// Type alias for representing episode numbers
pub type EpisodeNumber = u32;

/// Potential errors when building EpisodeNumbers
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// There are no episodes
	#[error("zero episodes")]
	Zero,
	/// There are gaps in the episodes
	#[error("noncontiguous episodes")]
	Noncontiguous,
}

/// A wrapper for handling single and multi-episode entries
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EpisodeNumbers(RangeInclusive<EpisodeNumber>);

impl TryFrom<&[EpisodeNumber]> for EpisodeNumbers {
	type Error = Error;

	fn try_from(value: &[EpisodeNumber]) -> Result<Self, Self::Error> {
		match value {
			[] => Err(Error::Zero),
			[n] => Ok(Self(*n..=*n)),
			_ => {
				// min and max will always exist
				let min = value.iter().copied().min().unwrap_or_default();
				let max = value.iter().copied().max().unwrap_or_default();
				let len = value.len();

				if usize::try_from(max.saturating_sub(min).saturating_add(1)) != Ok(len) {
					return Err(Error::Noncontiguous);
				}

				let set: HashSet<_> = value.iter().copied().collect();
				if set.len() != len {
					return Err(Error::Noncontiguous);
				}

				Ok(Self(min..=max))
			}
		}
	}
}

impl EpisodeNumbers {
	/// Get the range of episodes
	pub fn as_range(&self) -> &RangeInclusive<EpisodeNumber> {
		&self.0
	}
}
