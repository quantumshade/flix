use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{date, integer};

use crate::migration::m_000001::FlixInfoShows;

use super::super::m_000001::FlixInfoSeasons;

use super::FlixTmdbShows;

#[derive(Iden)]
pub enum FlixTmdbSeasons {
	Table,
	TmdbShow,
	TmdbSeason,
	FlixShow,
	FlixSeason,
	LastUpdate,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixTmdbSeasons::Table)
				.col(integer(FlixTmdbSeasons::TmdbShow))
				.col(integer(FlixTmdbSeasons::TmdbSeason))
				.col(integer(FlixTmdbSeasons::FlixShow))
				.col(integer(FlixTmdbSeasons::FlixSeason))
				.col(date(FlixTmdbSeasons::LastUpdate))
				.primary_key(
					Index::create()
						.col(FlixTmdbSeasons::TmdbShow)
						.col(FlixTmdbSeasons::TmdbSeason),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_tmdb_seasons-tmdb_show")
						.from_tbl(FlixTmdbSeasons::Table)
						.from_col(FlixTmdbSeasons::FlixShow)
						.to_tbl(FlixTmdbShows::Table)
						.to_col(FlixTmdbShows::FlixId),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_tmdb_seasons-flix_show")
						.from_tbl(FlixTmdbSeasons::Table)
						.from_col(FlixTmdbSeasons::FlixShow)
						.to_tbl(FlixInfoShows::Table)
						.to_col(FlixInfoShows::Id),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_tmdb_seasons-flix_show_season")
						.from_tbl(FlixTmdbSeasons::Table)
						.from_col(FlixTmdbSeasons::FlixShow)
						.from_col(FlixTmdbSeasons::FlixSeason)
						.to_tbl(FlixInfoSeasons::Table)
						.to_col(FlixInfoSeasons::Show)
						.to_col(FlixInfoSeasons::Season),
				)
				.index(
					Index::create()
						.unique()
						.name("idx-flix_tmdb_seasons-flix_show_season")
						.col(FlixTmdbSeasons::FlixShow)
						.col(FlixTmdbSeasons::FlixSeason),
				)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixTmdbSeasons::Table).to_owned())
		.await
}
