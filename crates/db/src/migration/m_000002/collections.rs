use seamantic::schema::sqlite_rowid_alias;

use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{date, integer};

use super::super::m_000001::FlixInfoCollections;

#[derive(Iden)]
pub enum FlixTmdbCollections {
	Table,
	TmdbId,
	FlixId,
	LastUpdate,
	MovieCount,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixTmdbCollections::Table)
				.col(sqlite_rowid_alias(FlixTmdbCollections::TmdbId))
				.col(integer(FlixTmdbCollections::FlixId).unique_key())
				.col(date(FlixTmdbCollections::LastUpdate))
				.col(integer(FlixTmdbCollections::MovieCount))
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_tmdb_collections-flixid")
						.from_tbl(FlixTmdbCollections::Table)
						.from_col(FlixTmdbCollections::FlixId)
						.to_tbl(FlixInfoCollections::Table)
						.to_col(FlixInfoCollections::Id),
				)
				.to_owned(),
		)
		.await?;

	manager
		.create_index(
			Index::create()
				.name("idx-flix_tmdb_collections-flixid")
				.table(FlixTmdbCollections::Table)
				.col(FlixTmdbCollections::FlixId)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixTmdbCollections::Table).to_owned())
		.await
}
