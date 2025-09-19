//! Show entity

use flix_model::id::ShowId;

use chrono::NaiveDate;
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};

/// The database representation of a flix show
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_info_shows")]
pub struct Model {
	/// The show's ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: ShowId,
	/// The show's title
	pub title: String,
	/// The show's tagline
	pub tagline: String,
	/// The show's overview
	pub overview: String,
	/// The show's air date
	pub date: NaiveDate,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	/// The seasons that are part of this show
	#[sea_orm(has_many = "super::seasons::Entity")]
	Seasons,
	/// The episodes that are part of this show
	#[sea_orm(has_many = "super::episodes::Entity")]
	Episodes,
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
