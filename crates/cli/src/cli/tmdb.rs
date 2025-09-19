use flix::model::numbers::{EpisodeNumber, SeasonNumber};
use flix::tmdb::model::id::RawId;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
	/// Process a TMDB collection
	Collection {
		#[arg(value_name = "TMDB_ID")]
		id: RawId,
	},
	/// Process a TMDB movie
	Movie {
		#[arg(value_name = "TMDB_ID")]
		id: RawId,
	},
	/// Process a TMDB show
	Show {
		#[arg(value_name = "TMDB_ID")]
		id: RawId,
	},
	/// Process a TMDB season
	Season {
		#[arg(value_name = "TMDB_ID")]
		id: RawId,
		#[arg(value_name = "SEASON_NUM")]
		season: SeasonNumber,
	},
	/// Process a TMDB episode
	#[command(trailing_var_arg = true)]
	Episode {
		#[arg(value_name = "TMDB_ID")]
		id: RawId,
		#[arg(value_name = "SEASON_NUM")]
		season: SeasonNumber,
		#[arg(value_name = "EPISODE_NUM")]
		episode: EpisodeNumber,
		#[arg(value_name = "...")]
		episodes: Vec<EpisodeNumber>,
	},
}
