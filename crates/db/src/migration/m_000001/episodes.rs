use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{date, integer, string};

use crate::migration::m_000001::FlixInfoShows;

use super::FlixInfoSeasons;

#[derive(Iden)]
pub enum FlixInfoEpisodes {
	Table,
	Show,
	Season,
	Episode,
	Title,
	Overview,
	Date,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixInfoEpisodes::Table)
				.col(integer(FlixInfoEpisodes::Show))
				.col(integer(FlixInfoEpisodes::Season))
				.col(integer(FlixInfoEpisodes::Episode))
				.col(string(FlixInfoEpisodes::Title))
				.col(string(FlixInfoEpisodes::Overview))
				.col(date(FlixInfoEpisodes::Date))
				.primary_key(
					Index::create()
						.col(FlixInfoEpisodes::Show)
						.col(FlixInfoEpisodes::Season)
						.col(FlixInfoEpisodes::Episode),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_info_episodes-show")
						.from_tbl(FlixInfoEpisodes::Table)
						.from_col(FlixInfoEpisodes::Show)
						.to_tbl(FlixInfoShows::Table)
						.to_col(FlixInfoShows::Id),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_info_episodes-show_season")
						.from_tbl(FlixInfoEpisodes::Table)
						.from_col(FlixInfoEpisodes::Show)
						.from_col(FlixInfoEpisodes::Season)
						.to_tbl(FlixInfoSeasons::Table)
						.to_col(FlixInfoSeasons::Show)
						.to_col(FlixInfoSeasons::Season),
				)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixInfoEpisodes::Table).to_owned())
		.await
}
