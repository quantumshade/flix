//! Filesystem scan result items.

use std::path::PathBuf;

use crate::Error;

/// An item returned by scanner streams.
#[derive(Debug)]
#[expect(clippy::exhaustive_structs, reason = "unlikely to change")]
pub struct Item<T> {
	/// The path of the item.
	pub path: PathBuf,
	/// The event relating to the item.
	pub event: Result<T, Error>,
}

impl<T> Item<T> {
	/// Helper function for mapping the inner event [Result].
	#[inline]
	pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> Item<U> {
		Item {
			path: self.path,
			event: self.event.map(op),
		}
	}
}
