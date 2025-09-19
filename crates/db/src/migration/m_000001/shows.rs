use seamantic::schema::sqlite_rowid_alias;

use sea_orm::sea_query;
use sea_orm::sea_query::{Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{date, string};

#[derive(Iden)]
pub enum FlixInfoShows {
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
				.table(FlixInfoShows::Table)
				.col(sqlite_rowid_alias(FlixInfoShows::Id))
				.col(string(FlixInfoShows::Title))
				.col(string(FlixInfoShows::Tagline))
				.col(string(FlixInfoShows::Overview))
				.col(date(FlixInfoShows::Date))
				.to_owned(),
		)
		.await?;

	manager
		.create_index(
			Index::create()
				.name("idx-flix_info_shows-title")
				.table(FlixInfoShows::Table)
				.col(FlixInfoShows::Title)
				.to_owned(),
		)
		.await?;

	manager
		.create_index(
			Index::create()
				.name("idx-flix_info_shows-date")
				.table(FlixInfoShows::Table)
				.col(FlixInfoShows::Date)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixInfoShows::Table).to_owned())
		.await
}
