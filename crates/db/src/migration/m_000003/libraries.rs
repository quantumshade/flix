use seamantic::schema::sqlite_rowid_alias;

use sea_orm::sea_query;
use sea_orm::sea_query::Table;
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::binary;

#[derive(Iden)]
pub enum FlixLibraries {
	Table,
	Id,
	Directory,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixLibraries::Table)
				.col(sqlite_rowid_alias(FlixLibraries::Id))
				.col(binary(FlixLibraries::Directory))
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixLibraries::Table).to_owned())
		.await
}
