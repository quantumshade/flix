//! CLI configuration.

/// Top level config struct.
#[derive(serde::Deserialize)]
pub(crate) struct Config {
	/// The TMDB config.
	tmdb: TmdbConfig,
}

/// TMDB config struct.
#[derive(serde::Deserialize)]
pub(crate) struct TmdbConfig {
	/// The bearer token to use for API requests.
	bearer_token: String,
}

impl Config {
	/// Get the TMDB config.
	pub(crate) const fn tmdb(&self) -> &TmdbConfig {
		&self.tmdb
	}
}

impl TmdbConfig {
	/// Get the bearer token.
	pub(crate) fn bearer_token(&self) -> &str {
		&self.bearer_token
	}
}
