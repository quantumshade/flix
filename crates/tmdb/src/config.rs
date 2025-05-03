use url::Url;
use url_macro::url;

/// The client configuration
pub struct Config {
	/// The base URL of the API
	pub base_url: Url,
	/// The reqwest client that is used for every request
	pub client: reqwest::Client,
	/// The bearer token for readonly access to the API
	pub bearer_token: String,
	/// An optional user agent string to provide to the API
	pub user_agent: Option<String>,
}

impl Config {
	/// Create a new configuration using the provided bearer token
	pub fn new(bearer_token: String) -> Self {
		Self {
			base_url: url!("https://api.themoviedb.org"),
			client: reqwest::Client::new(),
			bearer_token,
			user_agent: None,
		}
	}
}
