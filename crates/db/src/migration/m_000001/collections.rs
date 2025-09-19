use seamantic::schema::sqlite_rowid_alias;

use sea_orm::sea_query;
use sea_orm::sea_query::{Index, Table};
use sea_orm::{DbErr, Iden};
use sea_orm_migration::SchemaManager;
use sea_orm_migration::schema::string;

#[derive(Iden)]
pub enum FlixInfoCollections {
	Table,
	Id,
	Title,
	Overview,
}

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.create_table(
			Table::create()
				.table(FlixInfoCollections::Table)
				.col(sqlite_rowid_alias(FlixInfoCollections::Id))
				.col(string(FlixInfoCollections::Title))
				.col(string(FlixInfoCollections::Overview))
				.to_owned(),
		)
		.await?;

	manager
		.create_index(
			Index::create()
				.name("idx-flix_info_collections-title")
				.table(FlixInfoCollections::Table)
				.col(FlixInfoCollections::Title)
				.to_owned(),
		)
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table(FlixInfoCollections::Table).to_owned())
		.await
}
