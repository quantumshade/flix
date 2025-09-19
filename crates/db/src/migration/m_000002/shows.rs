use seamantic::schema::sqlite_rowid_alias;

use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{date, integer};

use super::super::m_000001::FlixInfoShows;

#[derive(Iden)]
pub enum FlixTmdbShows {
	Table,
	TmdbId,
	FlixId,
	LastUpdate,
	NumberOfSeasons,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixTmdbShows::Table)
				.col(sqlite_rowid_alias(FlixTmdbShows::TmdbId))
				.col(integer(FlixTmdbShows::FlixId).unique_key())
				.col(date(FlixTmdbShows::LastUpdate))
				.col(integer(FlixTmdbShows::NumberOfSeasons))
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_tmdb_shows-flixid")
						.from_tbl(FlixTmdbShows::Table)
						.from_col(FlixTmdbShows::FlixId)
						.to_tbl(FlixInfoShows::Table)
						.to_col(FlixInfoShows::Id),
				)
				.to_owned(),
		)
		.await?;

	manager
		.create_index(
			Index::create()
				.name("idx-flix_tmdb_shows-flixid")
				.table(FlixTmdbShows::Table)
				.col(FlixTmdbShows::FlixId)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixTmdbShows::Table).to_owned())
		.await
}
