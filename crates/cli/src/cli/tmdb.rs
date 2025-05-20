use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
	/// Process a TMDB collection
	Collection {
		#[arg(value_name = "TMDB_ID")]
		id: u32,
	},
	/// Process a TMDB movie
	Movie {
		#[arg(value_name = "TMDB_ID")]
		id: u32,
	},
	/// Process a TMDB show
	Show {
		#[arg(value_name = "TMDB_ID")]
		id: u32,
	},
	/// Process a TMDB season
	Season {
		#[arg(value_name = "TMDB_ID")]
		id: u32,
		#[arg(value_name = "SEASON_NUM")]
		season: u32,
	},
	/// Process a TMDB episode
	#[command(trailing_var_arg = true)]
	Episode {
		#[arg(value_name = "TMDB_ID")]
		id: u32,
		#[arg(value_name = "SEASON_NUM")]
		season: u32,
		#[arg(value_name = "EPISODE_NUM")]
		episode: u32,
		#[arg(value_name = "...")]
		episodes: Vec<u32>,
	},
}
