//! Episode entity

use flix_model::id::ShowId;

use chrono::NaiveDate;
use flix_model::numbers::{EpisodeNumber, SeasonNumber};
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};

/// The database representation of a flix episode
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_info_episodes")]
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
	/// The episode's title
	pub title: String,
	/// The episode's overview
	pub overview: String,
	/// The episode's air date
	pub date: NaiveDate,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	/// The show this season belongs to
	#[sea_orm(
		belongs_to = "super::shows::Entity",
		from = "Column::Show",
		to = "super::shows::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Show,
}

impl Related<super::shows::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Show.def()
	}
}
