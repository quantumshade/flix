use seamantic::schema::sqlite_rowid_alias;

use sea_orm::sea_query;
use sea_orm::sea_query::{Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{date, string};

#[derive(Iden)]
pub enum FlixInfoMovies {
	Table,
	Id,
	Title,
	Tagline,
	Overview,
	Date,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixInfoMovies::Table)
				.col(sqlite_rowid_alias(FlixInfoMovies::Id))
				.col(string(FlixInfoMovies::Title))
				.col(string(FlixInfoMovies::Tagline))
				.col(string(FlixInfoMovies::Overview))
				.col(date(FlixInfoMovies::Date))
				.to_owned(),
		)
		.await?;

	manager
		.create_index(
			Index::create()
				.name("idx-flix_info_movies-title")
				.table(FlixInfoMovies::Table)
				.col(FlixInfoMovies::Title)
				.to_owned(),
		)
		.await?;

	manager
		.create_index(
			Index::create()
				.name("idx-flix_info_movies-date")
				.table(FlixInfoMovies::Table)
				.col(FlixInfoMovies::Date)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixInfoMovies::Table).to_owned())
		.await
}
