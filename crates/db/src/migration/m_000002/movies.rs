use seamantic::schema::sqlite_rowid_alias;

use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{date, integer, integer_null};

use crate::migration::m_000002::FlixTmdbCollections;

use super::super::m_000001::FlixInfoMovies;

#[derive(Iden)]
pub enum FlixTmdbMovies {
	Table,
	TmdbId,
	FlixId,
	LastUpdate,
	Runtime,
	Collection,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixTmdbMovies::Table)
				.col(sqlite_rowid_alias(FlixTmdbMovies::TmdbId))
				.col(integer(FlixTmdbMovies::FlixId).unique_key())
				.col(date(FlixTmdbMovies::LastUpdate))
				.col(integer(FlixTmdbMovies::Runtime))
				.col(integer_null(FlixTmdbMovies::Collection))
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_tmdb_movies-flixid")
						.from_tbl(FlixTmdbMovies::Table)
						.from_col(FlixTmdbMovies::FlixId)
						.to_tbl(FlixInfoMovies::Table)
						.to_col(FlixInfoMovies::Id),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_tmdb_movies-collectionid")
						.from_tbl(FlixTmdbMovies::Table)
						.from_col(FlixTmdbMovies::Collection)
						.to_tbl(FlixTmdbCollections::Table)
						.to_col(FlixTmdbCollections::TmdbId),
				)
				.to_owned(),
		)
		.await?;

	manager
		.create_index(
			Index::create()
				.name("idx-flix_tmdb_movies-flixid")
				.table(FlixTmdbMovies::Table)
				.col(FlixTmdbMovies::FlixId)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixTmdbMovies::Table).to_owned())
		.await
}
