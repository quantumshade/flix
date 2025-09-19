use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{date, integer};

use super::super::m_000001::{FlixInfoEpisodes, FlixInfoSeasons};

use super::FlixTmdbSeasons;

#[derive(Iden)]
pub enum FlixTmdbEpisodes {
	Table,
	TmdbShow,
	TmdbSeason,
	TmdbEpisode,
	FlixShow,
	FlixSeason,
	FlixEpisode,
	LastUpdate,
	Runtime,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixTmdbEpisodes::Table)
				.col(integer(FlixTmdbEpisodes::TmdbShow))
				.col(integer(FlixTmdbEpisodes::TmdbSeason))
				.col(integer(FlixTmdbEpisodes::TmdbEpisode))
				.col(integer(FlixTmdbEpisodes::FlixShow))
				.col(integer(FlixTmdbEpisodes::FlixSeason))
				.col(integer(FlixTmdbEpisodes::FlixEpisode))
				.col(date(FlixTmdbEpisodes::LastUpdate))
				.col(integer(FlixTmdbEpisodes::Runtime))
				.primary_key(
					Index::create()
						.col(FlixTmdbEpisodes::TmdbShow)
						.col(FlixTmdbEpisodes::TmdbSeason)
						.col(FlixTmdbEpisodes::TmdbEpisode),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_tmdb_episodes-tmdb_show_season")
						.from_tbl(FlixTmdbEpisodes::Table)
						.from_col(FlixTmdbEpisodes::TmdbShow)
						.from_col(FlixTmdbEpisodes::TmdbSeason)
						.to_tbl(FlixTmdbSeasons::Table)
						.to_col(FlixTmdbSeasons::TmdbShow)
						.to_col(FlixTmdbSeasons::TmdbSeason),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_tmdb_episodes-flix_show_season")
						.from_tbl(FlixTmdbEpisodes::Table)
						.from_col(FlixTmdbEpisodes::FlixShow)
						.from_col(FlixTmdbEpisodes::FlixSeason)
						.to_tbl(FlixInfoSeasons::Table)
						.to_col(FlixInfoSeasons::Show)
						.to_col(FlixInfoSeasons::Season),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_tmdb_episodes-flix_show_season_episode")
						.from_tbl(FlixTmdbEpisodes::Table)
						.from_col(FlixTmdbEpisodes::FlixShow)
						.from_col(FlixTmdbEpisodes::FlixSeason)
						.from_col(FlixTmdbEpisodes::FlixEpisode)
						.to_tbl(FlixInfoEpisodes::Table)
						.to_col(FlixInfoEpisodes::Show)
						.to_col(FlixInfoEpisodes::Season)
						.to_col(FlixInfoEpisodes::Episode),
				)
				.index(
					Index::create()
						.unique()
						.name("idx-flix_tmdb_episodes-flix_show_season_episode")
						.col(FlixTmdbEpisodes::FlixShow)
						.col(FlixTmdbEpisodes::FlixSeason)
						.col(FlixTmdbEpisodes::FlixEpisode),
				)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixTmdbEpisodes::Table).to_owned())
		.await
}
