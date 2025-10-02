use seamantic::schema::sqlite_rowid_alias;

use sea_orm::sea_query;
use sea_orm::sea_query::{ForeignKeyCreateStatement, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::{binary, integer, integer_null, string, string_null};

use crate::migration::m_000001::FlixInfoMovies;
use crate::migration::m_000003::{FlixCollections, FlixLibraries};

#[derive(Iden)]
pub enum FlixMovies {
	Table,
	Id,
	Parent,
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
				.table(FlixMovies::Table)
				.col(sqlite_rowid_alias(FlixMovies::Id))
				.col(integer_null(FlixMovies::Parent))
				.col(string(FlixMovies::Slug))
				.col(integer(FlixMovies::Library))
				.col(binary(FlixMovies::Directory))
				.col(string(FlixMovies::RelativeMediaPath))
				.col(string_null(FlixMovies::RelativePosterPath))
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_movies-id")
						.from_tbl(FlixMovies::Table)
						.from_col(FlixMovies::Id)
						.to_tbl(FlixInfoMovies::Table)
						.to_col(FlixInfoMovies::Id),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_movies-parent")
						.from_tbl(FlixMovies::Table)
						.from_col(FlixMovies::Parent)
						.to_tbl(FlixCollections::Table)
						.to_col(FlixCollections::Id),
				)
				.foreign_key(
					ForeignKeyCreateStatement::new()
						.name("fk-flix_movies-library")
						.from_tbl(FlixMovies::Table)
						.from_col(FlixMovies::Library)
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
		.drop_table(Table::drop().table(FlixMovies::Table).to_owned())
		.await
}
