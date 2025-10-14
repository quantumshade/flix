//! Movie entity

use flix_model::id::{MovieId, RawId};

use chrono::NaiveDate;
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait,
};

/// The database representation of a watched movie
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_watched_movies")]
pub struct Model {
	/// The movie's ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: MovieId,
	/// The user's ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub user_id: RawId,
	/// The date this movie was watched
	pub watched_date: NaiveDate,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
