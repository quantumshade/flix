use sea_orm::DbErr;
use sea_orm::{ConnectionTrait, DbBackend, Statement};
use sea_orm_migration::SchemaManager;

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.get_connection()
		.execute_raw(Statement::from_string(
			DbBackend::Sqlite,
			r#"
				CREATE VIEW flix_watched_seasons AS
				SELECT
					w.show,
					w.season,
					w.user_id,
					MAX(w.watched_date) AS watched_date
				FROM flix_watched_episodes w
				WHERE NOT EXISTS (
					SELECT 1
					FROM flix_episodes e
					WHERE e.show = w.show
					AND e.season = w.season
					AND NOT EXISTS (
						SELECT 1
						FROM flix_watched_episodes wc
						WHERE wc.show = e.show
						AND wc.season = e.season
						AND wc.episode = e.episode
						AND wc.user_id = w.user_id
					)
				)
				GROUP BY w.show, w.season, w.user_id
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
