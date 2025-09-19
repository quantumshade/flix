use seamantic::schema::sqlite_rowid_alias;

use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{binary, binary_null, integer, integer_null, string};

use crate::migration::m_000001::FlixInfoShows;
use crate::migration::m_000003::{FlixCollections, FlixLibraries};

#[derive(Iden)]
pub enum FlixShows {
	Table,
	Id,
	Parent,
	Slug,
	Library,
	Directory,
	RelativePosterPath,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixShows::Table)
				.col(sqlite_rowid_alias(FlixShows::Id))
				.col(integer_null(FlixShows::Parent))
				.col(string(FlixShows::Slug))
				.col(integer(FlixShows::Library))
				.col(binary(FlixShows::Directory))
				.col(binary_null(FlixShows::RelativePosterPath))
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_shows-id")
						.from_tbl(FlixShows::Table)
						.from_col(FlixShows::Id)
						.to_tbl(FlixInfoShows::Table)
						.to_col(FlixInfoShows::Id),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_shows-parent")
						.from_tbl(FlixShows::Table)
						.from_col(FlixShows::Parent)
						.to_tbl(FlixCollections::Table)
						.to_col(FlixCollections::Id),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_shows-library")
						.from_tbl(FlixShows::Table)
						.from_col(FlixShows::Library)
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
		.drop_table(Table::drop().table(FlixShows::Table).to_owned())
		.await
}
