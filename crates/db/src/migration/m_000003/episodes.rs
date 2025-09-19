use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{binary, binary_null, integer, string};

use crate::migration::m_000001::FlixInfoEpisodes;
use crate::migration::m_000003::FlixLibraries;

#[derive(Iden)]
pub enum FlixEpisodes {
	Table,
	Show,
	Season,
	Episode,
	Slug,
	Library,
	Directory,
	RelativeMediaPath,
	RelativePosterPath,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixEpisodes::Table)
				.col(integer(FlixEpisodes::Show))
				.col(integer(FlixEpisodes::Season))
				.col(integer(FlixEpisodes::Episode))
				.col(string(FlixEpisodes::Slug))
				.col(integer(FlixEpisodes::Library))
				.col(binary(FlixEpisodes::Directory))
				.col(binary(FlixEpisodes::RelativeMediaPath))
				.col(binary_null(FlixEpisodes::RelativePosterPath))
				.primary_key(
					Index::create()
						.col(FlixEpisodes::Show)
						.col(FlixEpisodes::Season)
						.col(FlixEpisodes::Episode),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_episodes-show_season_episode")
						.from_tbl(FlixEpisodes::Table)
						.from_col(FlixEpisodes::Show)
						.from_col(FlixEpisodes::Season)
						.from_col(FlixEpisodes::Episode)
						.to_tbl(FlixInfoEpisodes::Table)
						.to_col(FlixInfoEpisodes::Show)
						.to_col(FlixInfoEpisodes::Season)
						.to_col(FlixInfoEpisodes::Episode),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_episodes-library")
						.from_tbl(FlixEpisodes::Table)
						.from_col(FlixEpisodes::Library)
						.to_tbl(FlixLibraries::Table)
						.to_col(FlixLibraries::Id),
				)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixEpisodes::Table).to_owned())
		.await
}
