//! Movie entity

use flix_model::id::MovieId as FlixId;
use flix_tmdb::model::id::{CollectionId, MovieId};

use seamantic::model::duration::Seconds;

use chrono::NaiveDate;
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};

/// The database representation of a tmdb movie
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_tmdb_movies")]
pub struct Model {
	/// The movie's TMDB ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub tmdb_id: MovieId,
	/// The movie's ID
	pub flix_id: FlixId,
	/// The date of the last update
	pub last_update: NaiveDate,
	/// The movie's runtime in seconds
	pub runtime: Seconds,
	/// The TMDB ID of the collection this movie belongs to
	pub collection: Option<CollectionId>,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	/// The collection this movie belongs to
	#[sea_orm(
		belongs_to = "super::collections::Entity",
		from = "Column::Collection",
		to = "super::collections::Column::TmdbId",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Collection,
}

impl Related<super::collections::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Collection.def()
	}
}
