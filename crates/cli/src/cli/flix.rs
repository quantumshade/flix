use flix::model::numbers::{EpisodeNumber, SeasonNumber};

use chrono::NaiveDate;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum AddCommand {
	/// Add a flix collection
	Collection {
		#[arg(value_name = "TITLE")]
		title: String,
		#[arg(value_name = "OVERVIEW")]
		overview: String,
	},
	/// Add a flix episode
	Episode {
		#[arg(value_name = "SHOW_WEB_SLUG")]
		show_slug: String,
		#[arg(value_name = "NUMBER")]
		season_number: SeasonNumber,
		#[arg(value_name = "NUMBER")]
		episode_number: EpisodeNumber,
		#[arg(value_name = "TITLE")]
		title: String,
		#[arg(value_name = "OVERVIEW")]
		overview: String,
		#[arg(value_name = "DATE")]
		air_date: NaiveDate,
	},
}
