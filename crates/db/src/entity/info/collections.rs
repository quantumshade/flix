//! Collection entity

use flix_model::id::CollectionId;

use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait,
};

/// The database representation of a flix collection
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_info_collections")]
pub struct Model {
	/// The collection's ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: CollectionId,
	/// The collection's title
	pub title: String,
	/// The collection's overview
	pub overview: String,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
