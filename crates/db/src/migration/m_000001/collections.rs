use sea_orm::prelude::*;
use sea_orm::sea_query::Table;
use sea_orm::{ConnectionTrait, DbBackend, Statement};
use sea_orm_migration::prelude::*;

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
	manager
		.drop_table(Table::drop().table("flix_watched_collections").to_owned())
		.await?;

	manager
		.get_connection()
		.execute_raw(Statement::from_string(
			DbBackend::Sqlite,
			r#"
				CREATE VIEW flix_watched_collections AS
				WITH RECURSIVE
				watched_items AS (
					SELECT
						w.id,
						w.user_id,
						w.watched_date,
						'movie' AS type
					FROM flix_watched_movies w

					UNION ALL

					SELECT
						w.id,
						w.user_id,
						w.watched_date,
						'show' AS type
					FROM flix_watched_shows w
				),
				collection_items AS (
					SELECT
						m.parent_id,
						m.id,
						'movie' AS type
					FROM flix_movies m
					WHERE m.parent_id IS NOT NULL

					UNION ALL

					SELECT
						s.parent_id,
						s.id,
						'show' AS type
					FROM flix_shows s
					WHERE s.parent_id IS NOT NULL

					UNION ALL

					SELECT
						c.parent_id,
						ci.id,
						ci.type
					FROM collection_items ci
					JOIN flix_collections c
						ON c.id = ci.parent_id
				)
				SELECT
					ci.parent_id AS id,
					wi.user_id,
					MAX(wi.watched_date) AS watched_date
				FROM collection_items ci
				JOIN watched_items wi
					ON wi.id = ci.id
					AND wi.type = ci.type
				WHERE NOT EXISTS (
					SELECT 1
					FROM collection_items ci2
					WHERE ci2.parent_id = ci.parent_id
					AND NOT EXISTS (
						SELECT 1
						FROM watched_items wi2
						WHERE wi2.id = ci2.id
						AND wi2.type = ci2.type
						AND wi2.user_id = wi.user_id
					)
				)
				GROUP BY ci.parent_id, wi.user_id
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
				DROP VIEW flix_watched_collections
				;
			"#,
		))
		.await?;

	Ok(())
}
