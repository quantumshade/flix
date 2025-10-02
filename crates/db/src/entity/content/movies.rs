//! Movie entity

use flix_model::id::{CollectionId, LibraryId, MovieId};

use seamantic::model::path::PathBytes;

use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};

/// The database representation of a movie media folder
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_movies")]
pub struct Model {
	/// The movie's ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: MovieId,
	/// The movie's parent
	pub parent: Option<CollectionId>,
	/// The movie's slug
	pub slug: String,
	/// The movie's library
	pub library: LibraryId,
	/// The movie's directory
	pub directory: PathBytes,
	/// The movie's media path
	pub relative_media_path: String,
	/// The movie's poster path
	pub relative_poster_path: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	/// The parent collection of this collection
	#[sea_orm(
		belongs_to = "super::collections::Entity",
		from = "Column::Parent",
		to = "super::collections::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Parent,
	/// The library this movie belongs to
	#[sea_orm(
		belongs_to = "super::libraries::Entity",
		from = "Column::Library",
		to = "super::libraries::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Library,
	/// The media info for this movie
	#[sea_orm(
		belongs_to = "super::super::info::movies::Entity",
		from = "Column::Id",
		to = "super::super::info::movies::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	MediaInfo,
	/// The watched info for this movie
	#[sea_orm(
		belongs_to = "super::super::watched::movies::Entity",
		from = "Column::Id",
		to = "super::super::watched::movies::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	WatchInfo,
}

impl Related<super::collections::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Parent.def()
	}
}

impl Related<super::libraries::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Library.def()
	}
}

impl Related<super::super::info::movies::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::MediaInfo.def()
	}
}

impl Related<super::super::watched::movies::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::WatchInfo.def()
	}
}
