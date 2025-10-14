//! Season entity

use flix_model::id::ShowId as FlixId;
use flix_model::numbers::{EpisodeNumber, SeasonNumber};
use flix_tmdb::model::id::ShowId;
use seamantic::model::duration::Seconds;

use chrono::NaiveDate;
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait,
};

/// The database representation of a tmdb episode
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_tmdb_episodes")]
pub struct Model {
	/// The episode's show's TMDB ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub tmdb_show: ShowId,
	/// The episode's season's TMDB season number
	#[sea_orm(primary_key, auto_increment = false)]
	pub tmdb_season: SeasonNumber,
	/// The episode's TMDB episode number
	#[sea_orm(primary_key, auto_increment = false)]
	pub tmdb_episode: EpisodeNumber,
	/// The episode's show's ID
	pub flix_show: FlixId,
	/// The episode's season's number
	pub flix_season: SeasonNumber,
	/// The episode's number
	pub flix_episode: EpisodeNumber,
	/// The date of the last update
	pub last_update: NaiveDate,
	/// The episode's runtime in seconds
	pub runtime: Seconds,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
