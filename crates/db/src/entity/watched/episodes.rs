//! Episode entity

use flix_model::id::{RawId, ShowId};
use flix_model::numbers::{EpisodeNumber, SeasonNumber};

use chrono::NaiveDate;
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait,
};

/// The database representation of a watched movie
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_watched_episodes")]
pub struct Model {
	/// The episode's show's ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub show: ShowId,
	/// The episode's season's number
	#[sea_orm(primary_key, auto_increment = false)]
	pub season: SeasonNumber,
	/// The episode's number
	#[sea_orm(primary_key, auto_increment = false)]
	pub episode: EpisodeNumber,
	/// The user's ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub user_id: RawId,
	/// The date this episode was watched
	pub watched_date: NaiveDate,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
