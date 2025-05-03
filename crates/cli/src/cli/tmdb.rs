use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
	/// Process a TMDB collection
	Collection {
		#[arg(value_name = "TMDB_ID")]
		id: i32,
	},
	/// Process a TMDB movie
	Movie {
		#[arg(value_name = "TMDB_ID")]
		id: i32,
	},
	/// Process a TMDB show
	Show {
		#[arg(value_name = "TMDB_ID")]
		id: i32,
	},
	/// Process a TMDB season
	Season {
		#[arg(value_name = "TMDB_ID")]
		id: i32,
		#[arg(value_name = "SEASON_NUM")]
		season: i32,
	},
	/// Process a TMDB episode
	Episode {
		#[arg(value_name = "TMDB_ID")]
		id: i32,
		#[arg(value_name = "SEASON_NUM")]
		season: i32,
		#[arg(value_name = "EPISODE_NUM")]
		episode: i32,
	},
}
