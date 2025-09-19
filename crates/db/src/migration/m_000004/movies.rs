use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{date, integer};

use crate::migration::m_000001::FlixInfoMovies;

#[derive(Iden)]
pub enum FlixWatchedMovies {
	Table,
	Id,
	UserId,
	WatchedDate,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixWatchedMovies::Table)
				.col(integer(FlixWatchedMovies::Id))
				.col(integer(FlixWatchedMovies::UserId))
				.col(date(FlixWatchedMovies::WatchedDate))
				.primary_key(
					Index::create()
						.col(FlixWatchedMovies::Id)
						.col(FlixWatchedMovies::UserId),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_watched_movies-id")
						.from_tbl(FlixWatchedMovies::Table)
						.from_col(FlixWatchedMovies::Id)
						.to_tbl(FlixInfoMovies::Table)
						.to_col(FlixInfoMovies::Id),
				)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixWatchedMovies::Table).to_owned())
		.await
}
