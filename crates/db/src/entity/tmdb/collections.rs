//! Collection entity

use flix_model::id::CollectionId as FlixId;
use flix_tmdb::model::id::CollectionId;

use chrono::NaiveDate;
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};

/// The database representation of a tmdb collection
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_tmdb_collections")]
pub struct Model {
	/// The collection's TMDB ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub tmdb_id: CollectionId,
	/// The collection's ID
	pub flix_id: FlixId,
	/// The date of the last update
	pub last_update: NaiveDate,
	/// The number of movies in the collection
	pub movie_count: u16,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	/// The movies that are part of this collection
	#[sea_orm(has_many = "super::movies::Entity")]
	Movies,
}

impl Related<super::movies::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Movies.def()
	}
}
