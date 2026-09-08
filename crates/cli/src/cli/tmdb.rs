//! Command line argument parsing for the `tmdb` subcommand.

use flix::model::numbers::{EpisodeNumber, SeasonNumber};
use flix::tmdb::model::id::TmdbRepr;

use clap::Subcommand;

/// Subcommand for adding `tmdb` media.
#[derive(Subcommand)]
pub(crate) enum Command {
	/// Process a TMDB collection.
	Collection {
		/// The collection's ID.
		#[arg(value_name = "TMDB_ID")]
		id: TmdbRepr,
	},
	/// Process a TMDB movie.
	Movie {
		/// The movie's ID.
		#[arg(value_name = "TMDB_ID")]
		id: TmdbRepr,
	},
	/// Process a TMDB show.
	Show {
		/// The show's ID.
		#[arg(value_name = "TMDB_ID")]
		id: TmdbRepr,
	},
	/// Process a TMDB season.
	Season {
		/// The season's show's ID.
		#[arg(value_name = "TMDB_ID")]
		id: TmdbRepr,
		/// The season's number.
		#[arg(value_name = "SEASON_NUM")]
		season: SeasonNumber,
	},
	/// Process a TMDB episode.
	#[command(trailing_var_arg = true)]
	Episode {
		/// The episode's show's ID.
		#[arg(value_name = "TMDB_ID")]
		id: TmdbRepr,
		/// The episode's season's number.
		#[arg(value_name = "SEASON_NUM")]
		season: SeasonNumber,
		/// The episode's number.
		#[arg(value_name = "EPISODE_NUM")]
		episode: EpisodeNumber,
		/// Additional episode numbers for merged media files.
		#[arg(value_name = "...")]
		episodes: Vec<EpisodeNumber>,
	},
}
