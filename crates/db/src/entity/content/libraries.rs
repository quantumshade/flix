//! Library entity

use flix_model::id::LibraryId;

use seamantic::model::path::PathBytes;

use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};

/// The database representation of a library media folder
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_libraries")]
pub struct Model {
	/// The library's ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: LibraryId,
	/// The library's directory
	pub directory: PathBytes,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	/// All collections in this library
	#[sea_orm(has_many = "super::collections::Entity")]
	Collections,
	#[sea_orm(has_many = "super::movies::Entity")]
	/// All movies in this library
	Movies,
	#[sea_orm(has_many = "super::shows::Entity")]
	/// All shows in this library
	Shows,
	#[sea_orm(has_many = "super::seasons::Entity")]
	/// All seasons in this library
	Seasons,
	#[sea_orm(has_many = "super::episodes::Entity")]
	/// All episodes in this library
	Episodes,
}

impl Related<super::collections::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Collections.def()
	}
}

impl Related<super::movies::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Movies.def()
	}
}

impl Related<super::shows::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Shows.def()
	}
}

impl Related<super::seasons::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Seasons.def()
	}
}

impl Related<super::episodes::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Episodes.def()
	}
}
