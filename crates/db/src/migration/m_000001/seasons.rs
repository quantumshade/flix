use sea_orm::prelude::*;
use sea_orm::sea_query::Table;
use sea_orm::{ConnectionTrait, DbBackend, Statement};
use sea_orm_migration::prelude::*;

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table("flix_watched_seasons").to_owned())
		.await?;

	manager
		.get_connection()
		.execute_raw(Statement::from_string(
			DbBackend::Sqlite,
			r#"
				CREATE VIEW flix_watched_seasons AS
				SELECT
					w.show_id,
					w.season_number,
					w.user_id,
					MAX(w.watched_date) AS watched_date
				FROM flix_watched_episodes w
				WHERE NOT EXISTS (
					SELECT 1
					FROM flix_episodes e
					WHERE e.show_id = w.show_id
					AND e.season_number = w.season_number
					AND NOT EXISTS (
						SELECT 1
						FROM flix_watched_episodes wc
						WHERE wc.show_id = e.show_id
						AND wc.season_number = e.season_number
						AND wc.episode_number = e.episode_number
						AND wc.user_id = w.user_id
					)
				)
				GROUP BY w.show_id, w.season_number, w.user_id
				;
			"#,
		))
		.await?;

	Ok(())
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.get_connection()
		.execute_raw(Statement::from_string(
			DbBackend::Sqlite,
			r#"
				DROP VIEW flix_watched_seasons
				;
			"#,
		))
		.await?;

	Ok(())
}
