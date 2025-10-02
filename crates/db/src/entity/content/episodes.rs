//! Episode entity

use flix_model::id::{LibraryId, ShowId};

use seamantic::model::path::PathBytes;

use flix_model::numbers::{EpisodeNumber, SeasonNumber};
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};

/// The database representation of a episode media folder
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_episodes")]
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
	/// The number of additional contained episodes
	pub count: u8,
	/// The episode's slug
	pub slug: String,
	/// The episode's library
	pub library: LibraryId,
	/// The episode's directory
	pub directory: PathBytes,
	/// The episode's media path
	pub relative_media_path: String,
	/// The episode's poster path
	pub relative_poster_path: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	/// The library this episode belongs to
	#[sea_orm(
		belongs_to = "super::libraries::Entity",
		from = "Column::Library",
		to = "super::libraries::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Library,
	/// The media info for this episode
	#[sea_orm(
		belongs_to = "super::super::info::episodes::Entity",
		from = "(Column::Show, Column::Season, Column::Episode)",
		to = "(super::super::info::episodes::Column::Show, super::super::info::episodes::Column::Season, super::super::info::episodes::Column::Episode)",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	MediaInfo,
	/// The watched info for this episode
	#[sea_orm(
		belongs_to = "super::super::watched::episodes::Entity",
		from = "(Column::Show, Column::Season, Column::Episode)",
		to = "(super::super::watched::episodes::Column::Show, super::super::watched::episodes::Column::Season, super::super::watched::episodes::Column::Episode)",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	WatchInfo,
}

impl Related<super::libraries::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Library.def()
	}
}

impl Related<super::super::info::episodes::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::MediaInfo.def()
	}
}

impl Related<super::super::watched::episodes::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::WatchInfo.def()
	}
}
