//! Custom views for shows.

use sea_orm::prelude::*;
use sea_orm::sea_query::Table;
use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

/// # Errors
/// Database operations can fail.
pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table("flix_watched_shows").to_owned())
		.await?;

	_ = manager
		.get_connection()
		.execute_raw(Statement::from_string(
			DbBackend::Sqlite,
			"
				CREATE VIEW flix_watched_shows AS
				SELECT
					w.show_id as id,
					w.user_id,
					MAX(w.watched_date) AS watched_date
				FROM flix_watched_seasons w
				WHERE NOT EXISTS (
					SELECT 1
					FROM flix_seasons s
					WHERE s.show_id = w.show_id
					AND NOT EXISTS (
						SELECT 1
						FROM flix_watched_seasons wc
						WHERE wc.show_id = s.show_id
						AND wc.season_number = s.season_number
						AND wc.user_id = w.user_id
					)
				)
				GROUP BY w.show_id, w.user_id
				;
			",
		))
		.await?;

	Ok(())
}

/// # Errors
/// Database operations can fail.
pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	_ = manager
		.get_connection()
		.execute_raw(Statement::from_string(
			DbBackend::Sqlite,
			"
				DROP VIEW flix_watched_shows
				;
			",
		))
		.await?;

	Ok(())
}
