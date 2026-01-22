//! flix-tmdb provides clients and models for fetching data from TMDB

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod api;
pub mod model;

mod cache;
pub use cache::{Cache, CachePolicy, RedbCache};

mod client;
pub use client::Client;

mod config;
pub use config::Config;
