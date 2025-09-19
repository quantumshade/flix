use sea_orm::DbErr;
use sea_orm::{ConnectionTrait, DbBackend, Statement};
use sea_orm_migration::SchemaManager;

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.get_connection()
		.execute_raw(Statement::from_string(
			DbBackend::Sqlite,
			r#"
				CREATE VIEW flix_watched_shows AS
				SELECT
					w.show as id,
					w.user_id,
					MAX(w.watched_date) AS watched_date
				FROM flix_watched_seasons w
				WHERE NOT EXISTS (
					SELECT 1
					FROM flix_seasons s
					WHERE s.show = w.show
					AND NOT EXISTS (
						SELECT 1
						FROM flix_watched_seasons wc
						WHERE wc.show = s.show
						AND wc.season = s.season
						AND wc.user_id = w.user_id
					)
				)
				GROUP BY w.show, w.user_id
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
				DROP VIEW flix_watched_shows
				;
			"#,
		))
		.await?;

	Ok(())
}
