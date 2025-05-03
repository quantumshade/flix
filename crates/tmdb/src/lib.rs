//! flix-tmdb provides clients and models for fetching data from TMDB

/// TMDB API clients
pub mod api;
/// Deserializable types from the TMDB API
pub mod model;

mod client;
pub use client::Client;

mod config;
pub use config::Config;
