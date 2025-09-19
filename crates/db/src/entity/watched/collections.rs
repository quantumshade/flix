//! Collection entity

use flix_model::id::{CollectionId, RawId};

use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EnumIter,
	PrimaryKeyTrait,
};

/// The database representation of a watched movie
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_watched_collections")]
pub struct Model {
	/// The collection's ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: CollectionId,
	/// The user's ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub user_id: RawId,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
