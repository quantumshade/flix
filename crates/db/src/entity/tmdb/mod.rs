//! This module contains entities for storing dynamic data from TMDB

pub mod collections;

pub mod movies;

pub mod episodes;
pub mod seasons;
pub mod shows;

#[cfg(test)]
mod tests {
	use core::time::Duration;

	use flix_model::id::{CollectionId, MovieId, ShowId};
	use flix_tmdb::model::id::{
		CollectionId as TmdbCollectionId, MovieId as TmdbMovieId, ShowId as TmdbShowId,
	};

	use chrono::NaiveDate;
	use sea_orm::ActiveModelTrait;
	use sea_orm::ActiveValue::{NotSet, Set};
	use sea_orm::sqlx::error::ErrorKind;
	use sea_orm_migration::MigratorTrait;

	use crate::migration::Migrator;
	use crate::tests::new_memory_db;

	use super::super::tests::get_error_kind;
	use super::super::tests::notsettable;
	use super::super::tests::{
		make_flix_collection, make_flix_episode, make_flix_movie, make_flix_season, make_flix_show,
	};

	#[tokio::test]
	async fn test_inserts() {
		let db = new_memory_db().await;
		Migrator::up(&db, None).await.expect("up");

		// Collections
		macro_rules! assert_collection {
			($db:expr, $id:literal, $tid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_collection!(@insert, $db, $id, $tid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.tmdb_id, TmdbCollectionId::from_raw($tid));
				assert_eq!(model.flix_id, CollectionId::from_raw($id));
				assert_eq!(model.last_update, NaiveDate::from_yo_opt(2000, $tid).expect("NaiveDate::from_yo_opt"));
				assert_eq!(model.movie_count, $id);
			};
			($db:expr, $id:literal, $tid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_collection!(@insert, $db, $id, $tid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal, $tid:literal $(; $($skip:ident),+)?) => {
				super::collections::ActiveModel {
					tmdb_id: notsettable!(tmdb_id, TmdbCollectionId::from_raw($tid) $(, $($skip),+)?),
					flix_id: notsettable!(flix_id, CollectionId::from_raw($id) $(, $($skip),+)?),
					last_update: notsettable!(last_update, NaiveDate::from_yo_opt(2000, $tid).expect("NaiveDate::from_yo_opt") $(, $($skip),+)?),
					movie_count: notsettable!(movie_count, $id $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_collection!(&db, 1, 1, ForeignKeyViolation);
		make_flix_collection!(&db, 1);
		make_flix_collection!(&db, 2);
		make_flix_collection!(&db, 3);

		assert_collection!(&db, 1, 1, Success);
		assert_collection!(&db, 1, 1, UniqueViolation);
		assert_collection!(&db, 1, 2, UniqueViolation);
		assert_collection!(&db, 2, 1, UniqueViolation);
		assert_collection!(&db, 2, 2, Success);
		assert_collection!(&db, 3, 3, Success; tmdb_id);
		assert_collection!(&db, 4, 4, NotNullViolation; flix_id);
		assert_collection!(&db, 5, 5, NotNullViolation; last_update);
		assert_collection!(&db, 6, 6, NotNullViolation; movie_count);

		// Movies
		macro_rules! assert_movie {
			($db:expr, $id:literal, $tid:literal, $cid:expr, Success $(; $($skip:ident),+)?) => {
				let model = assert_movie!(@insert, $db, $id, $tid, $cid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.tmdb_id, TmdbMovieId::from_raw($tid));
				assert_eq!(model.flix_id, MovieId::from_raw($id));
				assert_eq!(model.last_update, NaiveDate::from_yo_opt(2000, $tid).expect("NaiveDate::from_yo_opt"));
				assert_eq!(model.runtime, Duration::from_secs($tid).into());
				assert_eq!(model.collection, $cid);
			};
			($db:expr, $id:literal, $tid:literal, $cid:expr, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_movie!(@insert, $db, $id, $tid, $cid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal, $tid:literal, $cid:expr $(; $($skip:ident),+)?) => {
				super::movies::ActiveModel {
					tmdb_id: notsettable!(tmdb_id, TmdbMovieId::from_raw($tid) $(, $($skip),+)?),
					flix_id: notsettable!(flix_id, MovieId::from_raw($id) $(, $($skip),+)?),
					last_update: notsettable!(last_update, NaiveDate::from_yo_opt(2000, $tid).expect("NaiveDate::from_yo_opt") $(, $($skip),+)?),
					runtime: notsettable!(runtime, Duration::from_secs($tid).into() $(, $($skip),+)?),
					collection: notsettable!(collection, $cid $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_movie!(
			&db,
			1,
			1,
			Some(TmdbCollectionId::from_raw(1)),
			ForeignKeyViolation
		);
		make_flix_movie!(&db, 1);
		make_flix_movie!(&db, 2);
		make_flix_movie!(&db, 3);

		assert_movie!(&db, 1, 1, Some(TmdbCollectionId::from_raw(1)), Success);
		assert_movie!(&db, 1, 1, None, UniqueViolation);
		assert_movie!(&db, 1, 2, None, UniqueViolation);
		assert_movie!(&db, 2, 1, None, UniqueViolation);
		assert_movie!(&db, 2, 2, Some(TmdbCollectionId::from_raw(1)), Success);
		assert_movie!(&db, 3, 3, None, Success; tmdb_id);
		assert_movie!(&db, 4, 4, None, NotNullViolation; flix_id);
		assert_movie!(&db, 5, 5, None, NotNullViolation; last_update);
		assert_movie!(&db, 6, 6, None, NotNullViolation; runtime);
		assert_movie!(&db, 7, 7, None, ForeignKeyViolation; collection); // Must be `Set(None)`

		// Shows
		macro_rules! assert_show {
			($db:expr, $id:literal, $tid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_show!(@insert, $db, $id, $tid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.tmdb_id, TmdbShowId::from_raw($tid));
				assert_eq!(model.flix_id, ShowId::from_raw($id));
				assert_eq!(model.last_update, NaiveDate::from_yo_opt(2000, $tid).expect("NaiveDate::from_yo_opt"));
				assert_eq!(model.number_of_seasons, $id);
			};
			($db:expr, $id:literal, $tid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_show!(@insert, $db, $id, $tid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(
					get_error_kind(model).expect("get_error_kind"),
					ErrorKind::$error
				);
			};
			(@insert, $db:expr, $id:literal, $tid:literal $(; $($skip:ident),+)?) => {
				super::shows::ActiveModel {
					tmdb_id: notsettable!(tmdb_id, TmdbShowId::from_raw($tid) $(, $($skip),+)?),
					flix_id: notsettable!(flix_id, ShowId::from_raw($id) $(, $($skip),+)?),
					last_update: notsettable!(last_update, NaiveDate::from_yo_opt(2000, $tid).expect("NaiveDate::from_yo_opt") $(, $($skip),+)?),
					number_of_seasons: notsettable!(number_of_seasons, $id $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_show!(&db, 1, 1, ForeignKeyViolation);
		make_flix_show!(&db, 1);
		make_flix_show!(&db, 2);
		make_flix_show!(&db, 3);

		assert_show!(&db, 1, 1, Success);
		assert_show!(&db, 1, 1, UniqueViolation);
		assert_show!(&db, 1, 2, UniqueViolation);
		assert_show!(&db, 2, 1, UniqueViolation);
		assert_show!(&db, 2, 2, Success);
		assert_show!(&db, 3, 3, Success; tmdb_id);
		assert_show!(&db, 4, 4, NotNullViolation; flix_id);
		assert_show!(&db, 5, 5, NotNullViolation; last_update);
		assert_show!(&db, 6, 6, NotNullViolation; number_of_seasons);

		// Seasons
		macro_rules! assert_season {
			($db:expr, $show:literal, $season:literal, $tshow:literal, $tseason:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_season!(@insert, $db, $show, $season, $tshow, $tseason $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.tmdb_show, TmdbShowId::from_raw($tshow));
				assert_eq!(model.tmdb_season, $tseason);
				assert_eq!(model.flix_show, ShowId::from_raw($show));
				assert_eq!(model.flix_season, $season);
				assert_eq!(model.last_update, NaiveDate::from_yo_opt(2000, $tshow).expect("NaiveDate::from_yo_opt"));
			};
			($db:expr, $show:literal, $season:literal, $tshow:literal, $tseason:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_season!(@insert, $db, $show, $season, $tshow, $tseason $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(
					get_error_kind(model).expect("get_error_kind"),
					ErrorKind::$error
				);
			};
			(@insert, $db:expr, $show:literal, $season:literal, $tshow:literal, $tseason:literal $(; $($skip:ident),+)?) => {
				super::seasons::ActiveModel {
					tmdb_show: notsettable!(tmdb_show, TmdbShowId::from_raw($tshow) $(, $($skip),+)?),
					tmdb_season: notsettable!(tmdb_season, $tseason $(, $($skip),+)?),
					flix_show: notsettable!(flix_show, ShowId::from_raw($show) $(, $($skip),+)?),
					flix_season: notsettable!(flix_season, $season $(, $($skip),+)?),
					last_update: notsettable!(last_update, NaiveDate::from_yo_opt(2000, $tshow).expect("NaiveDate::from_yo_opt") $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_season!(&db, 1, 1, 1, 1, ForeignKeyViolation);
		make_flix_season!(&db, 1, 1);
		make_flix_season!(&db, 1, 2);

		assert_season!(&db, 1, 1, 1, 1, Success);
		assert_season!(&db, 1, 1, 1, 1, UniqueViolation);
		assert_season!(&db, 1, 1, 2, 1, UniqueViolation);
		assert_season!(&db, 2, 1, 1, 1, UniqueViolation);
		assert_season!(&db, 1, 2, 1, 2, Success);
		assert_season!(&db, 1, 3, 1, 3, NotNullViolation; tmdb_show);
		assert_season!(&db, 1, 4, 1, 4, NotNullViolation; tmdb_season);
		assert_season!(&db, 1, 5, 1, 5, NotNullViolation; flix_show);
		assert_season!(&db, 1, 6, 1, 6, NotNullViolation; flix_season);
		assert_season!(&db, 1, 7, 1, 7, NotNullViolation; last_update);

		// Episodes
		macro_rules! assert_episode {
			($db:expr, $show:literal, $season:literal, $episode:literal, $tshow:literal, $tseason:literal, $tepisode:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_episode!(@insert, $db, $show, $season, $episode, $tshow, $tseason, $tepisode $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.tmdb_show, TmdbShowId::from_raw($tshow));
				assert_eq!(model.tmdb_season, $tseason);
				assert_eq!(model.tmdb_episode, $tepisode);
				assert_eq!(model.flix_show, ShowId::from_raw($show));
				assert_eq!(model.flix_season, $season);
				assert_eq!(model.flix_episode, $episode);
				assert_eq!(model.last_update, NaiveDate::from_yo_opt(2000, $tshow).expect("NaiveDate::from_yo_opt"));
				assert_eq!(model.runtime, Duration::from_secs($tshow).into());
			};
			($db:expr, $show:literal, $season:literal, $episode:literal, $tshow:literal, $tseason:literal, $tepisode:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_episode!(@insert, $db, $show, $season, $episode, $tshow, $tseason, $tepisode $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(
					get_error_kind(model).expect("get_error_kind"),
					ErrorKind::$error
				);
			};
			(@insert, $db:expr, $show:literal, $season:literal, $episode:literal, $tshow:literal, $tseason:literal, $tepisode:literal $(; $($skip:ident),+)?) => {
				super::episodes::ActiveModel {
					tmdb_show: notsettable!(tmdb_show, TmdbShowId::from_raw($tshow) $(, $($skip),+)?),
					tmdb_season: notsettable!(tmdb_season, $tseason $(, $($skip),+)?),
					tmdb_episode: notsettable!(tmdb_episode, $tepisode $(, $($skip),+)?),
					flix_show: notsettable!(flix_show, ShowId::from_raw($show) $(, $($skip),+)?),
					flix_season: notsettable!(flix_season, $season $(, $($skip),+)?),
					flix_episode: notsettable!(flix_episode, $episode $(, $($skip),+)?),
					last_update: notsettable!(last_update, NaiveDate::from_yo_opt(2000, $tshow).expect("NaiveDate::from_yo_opt") $(, $($skip),+)?),
					runtime: notsettable!(runtime, Duration::from_secs($tshow).into() $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_episode!(&db, 1, 1, 1, 1, 1, 1, ForeignKeyViolation);
		make_flix_episode!(&db, 1, 1, 1);
		make_flix_episode!(&db, 1, 1, 2);

		assert_episode!(&db, 1, 1, 1, 1, 1, 1, Success);
		assert_episode!(&db, 1, 1, 1, 1, 1, 1, UniqueViolation);
		assert_episode!(&db, 1, 1, 1, 1, 2, 1, UniqueViolation);
		assert_episode!(&db, 1, 1, 1, 2, 1, 1, UniqueViolation);
		assert_episode!(&db, 1, 2, 1, 1, 1, 1, UniqueViolation);
		assert_episode!(&db, 2, 1, 1, 1, 1, 1, UniqueViolation);
		assert_episode!(&db, 1, 1, 2, 1, 1, 2, Success);
		assert_episode!(&db, 1, 1, 3, 1, 1, 3, NotNullViolation; tmdb_show);
		assert_episode!(&db, 1, 1, 3, 1, 1, 4, NotNullViolation; tmdb_season);
		assert_episode!(&db, 1, 1, 3, 1, 1, 5, NotNullViolation; tmdb_episode);
		assert_episode!(&db, 1, 1, 3, 1, 1, 6, NotNullViolation; flix_show);
		assert_episode!(&db, 1, 1, 3, 1, 1, 7, NotNullViolation; flix_season);
		assert_episode!(&db, 1, 1, 3, 1, 1, 8, NotNullViolation; flix_episode);
		assert_episode!(&db, 1, 1, 3, 1, 1, 9, NotNullViolation; last_update);
		assert_episode!(&db, 1, 1, 3, 1, 1, 10, NotNullViolation; runtime);
	}
}
