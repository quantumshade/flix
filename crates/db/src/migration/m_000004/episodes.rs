use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{date, integer};

use crate::migration::m_000001::FlixInfoEpisodes;

#[derive(Iden)]
pub enum FlixWatchedEpisodes {
	Table,
	Show,
	Season,
	Episode,
	UserId,
	WatchedDate,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixWatchedEpisodes::Table)
				.col(integer(FlixWatchedEpisodes::Show))
				.col(integer(FlixWatchedEpisodes::Season))
				.col(integer(FlixWatchedEpisodes::Episode))
				.col(integer(FlixWatchedEpisodes::UserId))
				.col(date(FlixWatchedEpisodes::WatchedDate))
				.primary_key(
					Index::create()
						.col(FlixWatchedEpisodes::Show)
						.col(FlixWatchedEpisodes::Season)
						.col(FlixWatchedEpisodes::Episode)
						.col(FlixWatchedEpisodes::UserId),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_watched_episodes-show_season_episode")
						.from_tbl(FlixWatchedEpisodes::Table)
						.from_tbl(FlixWatchedEpisodes::Table)
						.from_col(FlixWatchedEpisodes::Show)
						.from_col(FlixWatchedEpisodes::Season)
						.from_col(FlixWatchedEpisodes::Episode)
						.to_tbl(FlixInfoEpisodes::Table)
						.to_col(FlixInfoEpisodes::Show)
						.to_col(FlixInfoEpisodes::Season)
						.to_col(FlixInfoEpisodes::Episode),
				)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixWatchedEpisodes::Table).to_owned())
		.await
}
