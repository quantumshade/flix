//! Season entity

use flix_model::id::{LibraryId, ShowId};

use seamantic::model::path::PathBytes;

use flix_model::numbers::SeasonNumber;
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};

/// The database representation of a season media folder
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_seasons")]
pub struct Model {
	/// The season's show's ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub show: ShowId,
	/// The season's number
	#[sea_orm(primary_key, auto_increment = false)]
	pub season: SeasonNumber,
	/// The season's slug
	pub slug: String,
	/// The season's library
	pub library: LibraryId,
	/// The season's directory
	pub directory: PathBytes,
	/// The season's poster path
	pub relative_poster_path: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	/// The library this season belongs to
	#[sea_orm(
		belongs_to = "super::libraries::Entity",
		from = "Column::Library",
		to = "super::libraries::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Library,
	/// The media info for this show
	#[sea_orm(
		belongs_to = "super::super::info::seasons::Entity",
		from = "(Column::Show, Column::Season)",
		to = "(super::super::info::seasons::Column::Show, super::super::info::seasons::Column::Season)",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	MediaInfo,
	/// The watched info for this show
	#[sea_orm(
		belongs_to = "super::super::watched::seasons::Entity",
		from = "(Column::Show, Column::Season)",
		to = "(super::super::watched::seasons::Column::Show, super::super::watched::seasons::Column::Season)",
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

impl Related<super::super::info::seasons::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::MediaInfo.def()
	}
}

impl Related<super::super::watched::seasons::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::WatchInfo.def()
	}
}
