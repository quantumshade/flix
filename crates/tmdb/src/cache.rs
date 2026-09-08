//! Caching related types.

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use bytes::Bytes;
use redb::{Database, DatabaseError, ReadableDatabase as _, TableDefinition};

/// The client cache policy.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum CachePolicy {
	/// Do not use a cache.
	None,
	/// Use and update the cache.
	Full,
	/// Use the cache but don't update it.
	Read,
	/// Ignore the cache but update it.
	Update,
}

/// The trait representing a caching backend.
pub trait Cache: core::fmt::Debug + Send + Sync {
	/// Get a cached value, or None.
	fn get(&self, query: &str) -> Option<Bytes>;
	/// Set a value in the cache.
	fn set(&self, query: &str, response: &Bytes);
}

/// The [`TableDefinition`] for TMDB response data.
const TABLE: TableDefinition<'_, &str, (u64, &[u8])> = TableDefinition::new("tmdb_responses");

/// A [Cache] implementation using [redb] as the backend.
#[derive(Debug)]
pub struct RedbCache {
	/// The underlying database.
	db: Database,
}

impl RedbCache {
	/// Create/open a [redb] database at the path.
	///
	/// # Errors
	/// Fails if creating/opening the database returns an error.
	#[inline]
	pub fn new(path: &Path) -> Result<Self, DatabaseError> {
		Ok(Self {
			db: Database::create(path)?,
		})
	}

	/// Helper function allowing for `.ok()?`.
	fn write(&self, timestamp: u64, query: &str, response: &Bytes) -> Option<()> {
		let write_txn = self.db.begin_write().ok()?;
		{
			let mut table = write_txn.open_table(TABLE).ok()?;
			drop(
				table
					.insert(query, (timestamp, response.iter().as_slice()))
					.ok()?,
			);
		}
		write_txn.commit().ok()
	}
}

impl Cache for RedbCache {
	#[inline]
	fn get(&self, query: &str) -> Option<Bytes> {
		let read_txn = self.db.begin_read().ok()?;
		let table = read_txn.open_table(TABLE).ok()?;

		let result = table.get(query).ok()??;
		let (timestamp, data) = result.value();

		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_or(0, |d| d.as_secs());

		if now.saturating_sub(timestamp) >= 60 * 60 * 24 * 30 * 6 {
			None
		} else {
			Some(Bytes::copy_from_slice(data))
		}
	}

	#[inline]
	fn set(&self, query: &str, response: &Bytes) {
		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_or(0, |d| d.as_secs());

		_ = self.write(now, query, response);
	}
}
