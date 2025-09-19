//! Show entity

use flix_model::id::ShowId as FlixId;
use flix_tmdb::model::id::ShowId;

use chrono::NaiveDate;
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};

/// The database representation of a tmdb show
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_tmdb_shows")]
pub struct Model {
	/// The show's TMDB ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub tmdb_id: ShowId,
	/// The show's ID
	pub flix_id: FlixId,
	/// The movie's runtime in seconds
	pub last_update: NaiveDate,
	/// The number of seasons the show has
	pub number_of_seasons: u32,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	/// The seasons that are part of this show
	#[sea_orm(has_many = "super::seasons::Entity")]
	Seasons,
}

impl Related<super::seasons::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Seasons.def()
	}
}
