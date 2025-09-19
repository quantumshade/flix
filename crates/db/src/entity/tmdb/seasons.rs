//! Season entity

use flix_model::id::ShowId as FlixId;
use flix_model::numbers::SeasonNumber;
use flix_tmdb::model::id::ShowId;

use chrono::NaiveDate;
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};

/// The database representation of a tmdb season
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_tmdb_seasons")]
pub struct Model {
	/// The season's show's TMDB ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub tmdb_show: ShowId,
	/// The season's TMDB season number
	#[sea_orm(primary_key, auto_increment = false)]
	pub tmdb_season: SeasonNumber,
	/// The season's show's ID
	pub flix_show: FlixId,
	/// The season's number
	pub flix_season: SeasonNumber,
	/// The date of the last update
	pub last_update: NaiveDate,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	/// The show this season belongs to
	#[sea_orm(
		belongs_to = "super::shows::Entity",
		from = "Column::TmdbShow",
		to = "super::shows::Column::TmdbId",
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
