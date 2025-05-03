#[derive(serde::Deserialize)]
pub struct Config {
	tmdb: TmdbConfig,
}

#[derive(serde::Deserialize)]
pub struct TmdbConfig {
	bearer_token: String,
}

impl Config {
	pub fn tmdb(&self) -> &TmdbConfig {
		&self.tmdb
	}
}

impl TmdbConfig {
	pub fn bearer_token(&self) -> &str {
		&self.bearer_token
	}
}
