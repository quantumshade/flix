use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{date, integer, string};

use super::FlixInfoShows;

#[derive(Iden)]
pub enum FlixInfoSeasons {
	Table,
	Show,
	Season,
	Title,
	Overview,
	Date,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixInfoSeasons::Table)
				.col(integer(FlixInfoSeasons::Show))
				.col(integer(FlixInfoSeasons::Season))
				.col(string(FlixInfoSeasons::Title))
				.col(string(FlixInfoSeasons::Overview))
				.col(date(FlixInfoSeasons::Date))
				.primary_key(
					Index::create()
						.col(FlixInfoSeasons::Show)
						.col(FlixInfoSeasons::Season),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_info_seasons-show")
						.from_tbl(FlixInfoSeasons::Table)
						.from_col(FlixInfoSeasons::Show)
						.to_tbl(FlixInfoShows::Table)
						.to_col(FlixInfoShows::Id),
				)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixInfoSeasons::Table).to_owned())
		.await
}
