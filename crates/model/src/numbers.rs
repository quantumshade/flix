//! This module contains season and episode numbers and related errors

use core::fmt;
use core::ops::RangeInclusive;
use core::str::FromStr;
use std::collections::HashSet;

use seamantic::sea_orm;

/// Newtype for representing season numbers
#[derive(
	Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, sea_orm::DeriveValueType,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct SeasonNumber(u32);

impl SeasonNumber {
	/// Create a `SeasonNumber` from an integer
	pub fn new(value: u32) -> Self {
		Self(value)
	}
}

impl fmt::Display for SeasonNumber {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl FromStr for SeasonNumber {
	type Err = <u32 as FromStr>::Err;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		u32::from_str(s).map(Self)
	}
}

/// Newtype for representing episode numbers
#[derive(
	Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, sea_orm::DeriveValueType,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct EpisodeNumber(u32);

impl EpisodeNumber {
	/// Create an `EpisodeNumber` from an integer
	pub fn new(value: u32) -> Self {
		Self(value)
	}

	/// Get the underlying value
	pub fn into_inner(self) -> u32 {
		self.0
	}
}

impl fmt::Display for EpisodeNumber {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl FromStr for EpisodeNumber {
	type Err = <u32 as FromStr>::Err;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		u32::from_str(s).map(Self)
	}
}

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
#[derive(Debug, Clone, PartialEq, Eq)]
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

				if usize::try_from(max.0.saturating_sub(min.0).saturating_add(1)) != Ok(len) {
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
	/// Create an [EpisodeNumbers] from a starting number and a count.
	/// `count` should be zero for single episodes.
	pub fn new(start: EpisodeNumber, count: u8) -> Self {
		Self(start..=EpisodeNumber(start.0.saturating_add(count.into())))
	}

	/// Get the range of episodes
	pub fn as_range(&self) -> &RangeInclusive<EpisodeNumber> {
		&self.0
	}

	/// Render this [EpisodeNumbers] as a range. If only one episode is
	/// is present it renders as `01`, if multiple it renders as `01-02`
	pub fn range_string(&self) -> String {
		let start = self.0.start();
		let end = self.0.end();

		if start == end {
			format!("{:02}", start)
		} else {
			format!("{:02}-{:02}", start, end)
		}
	}
}
