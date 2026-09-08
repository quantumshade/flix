//! flix-tmdb provides clients and models for fetching data from TMDB.

#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

pub mod api;
pub mod model;

mod cache;
pub use cache::{Cache, CachePolicy, RedbCache};

mod client;
pub use client::Client;

mod config;
pub use config::Config;

#[cfg(test)]
mod tests {
	use alloc::sync::Arc;

	use super::{CachePolicy, Client, Config, RedbCache};

	#[cfg(not(miri))]
	#[test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	fn use_types() {
		let tmp = tempfile::tempdir().expect("temporary directory exists");
		let config = Config::new(String::new());
		let cache = Arc::new(RedbCache::new(&tmp.path().join("cache.redb")).expect("redb"));
		drop(Client::new(config, cache, CachePolicy::Full));
	}
}
