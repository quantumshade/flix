//! Command line argument parsing for the `flix` subcommand.

use flix::model::numbers::{EpisodeNumber, SeasonNumber};

use chrono::NaiveDate;
use clap::Subcommand;

/// Subcommand for adding `flix` media.
#[derive(Subcommand)]
pub(crate) enum AddCommand {
	/// Add a flix collection.
	Collection {
		/// The collection's title.
		#[arg(value_name = "TITLE")]
		title: String,
		/// The collection's overview.
		#[arg(value_name = "OVERVIEW")]
		overview: String,
	},
	/// Add a flix episode.
	Episode {
		/// The episode's show's web slug.
		#[arg(value_name = "SHOW_WEB_SLUG")]
		show_web_slug: String,
		/// The episode's season number.
		#[arg(value_name = "NUMBER")]
		season_number: SeasonNumber,
		/// The episode's number.
		#[arg(value_name = "NUMBER")]
		episode_number: EpisodeNumber,
		/// The episode's title.
		#[arg(value_name = "TITLE")]
		title: String,
		/// The episode's overview.
		#[arg(value_name = "OVERVIEW")]
		overview: String,
		/// The episode's air date.
		#[arg(value_name = "DATE")]
		air_date: NaiveDate,
	},
}
