use seamantic::schema::sqlite_rowid_alias;

use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{binary, binary_null, integer, integer_null, string};

use crate::migration::m_000001::FlixInfoCollections;
use crate::migration::m_000003::FlixLibraries;

#[derive(Iden)]
pub enum FlixCollections {
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
				.table(FlixCollections::Table)
				.col(sqlite_rowid_alias(FlixCollections::Id))
				.col(integer_null(FlixCollections::Parent))
				.col(string(FlixCollections::Slug))
				.col(integer(FlixCollections::Library))
				.col(binary(FlixCollections::Directory))
				.col(binary_null(FlixCollections::RelativePosterPath))
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_collections-id")
						.from_tbl(FlixCollections::Table)
						.from_col(FlixCollections::Id)
						.to_tbl(FlixInfoCollections::Table)
						.to_col(FlixInfoCollections::Id),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_collections-parent")
						.from_tbl(FlixCollections::Table)
						.from_col(FlixCollections::Parent)
						.to_tbl(FlixCollections::Table)
						.to_col(FlixCollections::Id),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_collections-library")
						.from_tbl(FlixCollections::Table)
						.from_col(FlixCollections::Library)
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
		.drop_table(Table::drop().table(FlixCollections::Table).to_owned())
		.await
}
