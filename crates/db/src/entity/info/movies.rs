//! Movie entity

use flix_model::id::MovieId;

use chrono::NaiveDate;
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EnumIter,
	PrimaryKeyTrait,
};

/// The database representation of a flix movie
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "flix_info_movies")]
pub struct Model {
	/// The movie's ID
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: MovieId,
	/// The movie's title
	pub title: String,
	/// The movie's tagline
	pub tagline: String,
	/// The movie's overview
	pub overview: String,
	/// The movie's release date
	pub date: NaiveDate,
}

impl ActiveModelBehavior for ActiveModel {}

/// Relation
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
