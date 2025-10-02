use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{binary, integer, string, string_null};

use crate::migration::m_000001::FlixInfoSeasons;
use crate::migration::m_000003::FlixLibraries;

#[derive(Iden)]
pub enum FlixSeasons {
	Table,
	Show,
	Season,
	Slug,
	Library,
	Directory,
	RelativePosterPath,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixSeasons::Table)
				.col(integer(FlixSeasons::Show))
				.col(integer(FlixSeasons::Season))
				.col(string(FlixSeasons::Slug))
				.col(integer(FlixSeasons::Library))
				.col(binary(FlixSeasons::Directory))
				.col(string_null(FlixSeasons::RelativePosterPath))
				.primary_key(
					Index::create()
						.col(FlixSeasons::Show)
						.col(FlixSeasons::Season),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_seasons-show_season")
						.from_tbl(FlixSeasons::Table)
						.from_col(FlixSeasons::Show)
						.from_col(FlixSeasons::Season)
						.to_tbl(FlixInfoSeasons::Table)
						.to_col(FlixInfoSeasons::Show)
						.to_col(FlixInfoSeasons::Season),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_seasons-library")
						.from_tbl(FlixSeasons::Table)
						.from_col(FlixSeasons::Library)
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
		.drop_table(Table::drop().table(FlixSeasons::Table).to_owned())
		.await
}
