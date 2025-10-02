//! This module contains entities for storing watched information

pub mod collections;

pub mod movies;

pub mod episodes;
pub mod seasons;
pub mod shows;

#[cfg(test)]
mod tests {

	use flix_model::id::{MovieId, ShowId};

	use chrono::NaiveDate;
	use sea_orm::ActiveValue::{NotSet, Set};
	use sea_orm::sqlx::error::ErrorKind;
	use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter};
	use sea_orm_migration::MigratorTrait;

	use crate::entity::tests::{have_collection, have_library, have_movie, have_season, have_show};
	use crate::migration::Migrator;
	use crate::tests::new_memory_db;

	use super::super::tests::get_error_kind;
	use super::super::tests::notsettable;
	use super::super::tests::{
		have_episode, make_flix_episode, make_flix_movie, make_flix_season, make_flix_show,
	};

	macro_rules! assert_movie {
		($db:expr, $id:literal, $uid:literal, Success $(; $($skip:ident),+)?) => {
			let model = assert_movie!(@insert, $db, $id, $uid $(; $($skip),+)?)
				.expect("insert");

			assert_eq!(model.id, MovieId::from_raw($id));
			assert_eq!(model.user_id, $uid);
			assert_eq!(model.watched_date, NaiveDate::from_yo_opt(2000, $uid).expect("NaiveDate::from_yo_opt"));
		};
		($db:expr, $id:literal, $uid:literal, $error:ident $(; $($skip:ident),+)?) => {
			let model = assert_movie!(@insert, $db, $id, $uid $(; $($skip),+)?)
				.expect_err("insert");

			assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
		};
		(@insert, $db:expr, $id:literal, $uid:literal $(; $($skip:ident),+)?) => {
			super::movies::ActiveModel {
				id: notsettable!(id, MovieId::from_raw($id) $(, $($skip),+)?),
				user_id: notsettable!(user_id, $uid $(, $($skip),+)?),
				watched_date: notsettable!(watched_date, NaiveDate::from_yo_opt(2000, $uid).expect("NaiveDate::from_yo_opt") $(, $($skip),+)?),
			}.insert($db).await
		};
	}

	macro_rules! assert_episode {
		($db:expr, $show:literal, $season:literal, $episode:literal, $uid:literal, Success $(; $($skip:ident),+)?) => {
			let model = assert_episode!(@insert, $db, $show, $season, $episode, $uid $(; $($skip),+)?)
				.expect("insert");

			assert_eq!(model.show, ShowId::from_raw($show));
			assert_eq!(model.season, $season);
			assert_eq!(model.episode, $episode);
			assert_eq!(model.user_id, $uid);
			assert_eq!(model.watched_date, NaiveDate::from_yo_opt(2000, $uid).expect("NaiveDate::from_yo_opt"));
		};
		($db:expr, $show:literal, $season:literal, $episode:literal, $uid:literal, $error:ident $(; $($skip:ident),+)?) => {
			let model = assert_episode!(@insert, $db, $show, $season, $episode, $uid $(; $($skip),+)?)
				.expect_err("insert");

			assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
		};
		(@insert, $db:expr, $show:literal, $season:literal, $episode:literal, $uid:literal $(; $($skip:ident),+)?) => {
			super::episodes::ActiveModel {
				show: notsettable!(show, ShowId::from_raw($show) $(, $($skip),+)?),
				season: notsettable!(season, $season $(, $($skip),+)?),
				episode: notsettable!(episode, $episode $(, $($skip),+)?),
				user_id: notsettable!(user_id, $uid $(, $($skip),+)?),
				watched_date: notsettable!(watched_date, NaiveDate::from_yo_opt(2000, $uid).expect("NaiveDate::from_yo_opt") $(, $($skip),+)?),
			}.insert($db).await
		};
	}

	#[tokio::test]
	async fn test_inserts() {
		let db = new_memory_db().await;
		Migrator::up(&db, None).await.expect("up");

		// Movies
		assert_movie!(&db, 1, 1, ForeignKeyViolation);
		make_flix_movie!(&db, 1);
		make_flix_movie!(&db, 2);

		assert_movie!(&db, 1, 1, Success);
		assert_movie!(&db, 1, 1, UniqueViolation);
		assert_movie!(&db, 2, 1, Success);
		assert_movie!(&db, 3, 1, NotNullViolation; id);
		assert_movie!(&db, 4, 1, NotNullViolation; user_id);
		assert_movie!(&db, 5, 1, NotNullViolation; watched_date);

		// Episodes
		make_flix_show!(&db, 1);
		make_flix_season!(&db, 1, 1);
		make_flix_show!(&db, 2);
		make_flix_season!(&db, 2, 1);

		assert_episode!(&db, 1, 1, 1, 1, ForeignKeyViolation);
		make_flix_episode!(&db, 1, 1, 1);
		make_flix_episode!(&db, 1, 1, 2);
		make_flix_episode!(&db, 2, 1, 1);

		assert_episode!(&db, 1, 1, 1, 1, Success);
		assert_episode!(&db, 1, 1, 1, 1, UniqueViolation);
		assert_episode!(&db, 1, 1, 2, 1, Success);
		assert_episode!(&db, 2, 1, 1, 1, Success);
		assert_episode!(&db, 3, 1, 1, 1, NotNullViolation; show);
		assert_episode!(&db, 4, 1, 1, 1, NotNullViolation; season);
		assert_episode!(&db, 5, 1, 1, 1, NotNullViolation; episode);
		assert_episode!(&db, 6, 1, 1, 1, NotNullViolation; user_id);
		assert_episode!(&db, 7, 1, 1, 1, NotNullViolation; watched_date);
	}

	#[tokio::test]
	async fn test_queries() {
		let db = new_memory_db().await;
		Migrator::up(&db, None).await.expect("up");

		have_library!(&db, 1);

		// Season + Show
		macro_rules! assert_season {
			($db:expr, $show:literal, $season:literal, $uid:literal, Watched) => {
				assert_season!(@find, $db, $show, $season, $uid)
					.ok_or(())
					.expect("is none");
			};
			($db:expr, $show:literal, $season:literal, $uid:literal, Unwatched) => {
				assert_season!(@find, $db, $show, $season, $uid)
					.ok_or(())
					.expect_err("is some");
			};
			(@find, $db:expr, $show:literal, $season:literal, $uid:literal) => {
				super::seasons::Entity::find()
					.filter(
						Condition::all()
							.add(super::seasons::Column::Show.eq($show))
							.add(super::seasons::Column::Season.eq($season))
							.add(super::seasons::Column::UserId.eq($uid)),
					)
					.one(&db)
					.await
					.expect("find.filter.one")
			};
		}
		macro_rules! assert_show {
			($db:expr, $show:literal, $uid:literal, Watched) => {
				assert_show!(@find, $db, $show, $uid)
					.ok_or(())
					.expect("is none");
			};
			($db:expr, $show:literal, $uid:literal, Unwatched) => {
				assert_show!(@find, $db, $show, $uid)
					.ok_or(())
					.expect_err("is some");
			};
			(@find, $db:expr, $show:literal, $uid:literal) => {
				super::shows::Entity::find()
					.filter(
						Condition::all()
							.add(super::shows::Column::Id.eq($show))
							.add(super::shows::Column::UserId.eq($uid)),
					)
					.one(&db)
					.await
					.expect("find.filter.one")
			};
		}

		have_show!(&db, 1, 1, None);
		have_season!(&db, 1, 1, 1);
		have_episode!(&db, 1, 1, 1, 1);
		assert_episode!(&db, 1, 1, 1, 1, Success);
		assert_episode!(&db, 1, 1, 1, 2, Success);
		have_episode!(&db, 1, 1, 1, 2, >1); // Covers 2 and 3
		make_flix_episode!(&db, 1, 1, 3);
		assert_episode!(&db, 1, 1, 2, 1, Success);
		assert_episode!(&db, 1, 1, 2, 2, Success);
		have_season!(&db, 1, 1, 2);
		have_episode!(&db, 1, 1, 2, 1);
		assert_episode!(&db, 1, 2, 1, 1, Success);
		assert_episode!(&db, 1, 2, 1, 2, Success);
		have_episode!(&db, 1, 1, 2, 2);
		assert_episode!(&db, 1, 2, 2, 1, Success);
		have_episode!(&db, 1, 1, 2, 3);
		assert_episode!(&db, 1, 2, 3, 1, Success);

		// Add watched episodes that we do not have
		make_flix_episode!(&db, 1, 1, 4);
		assert_episode!(&db, 1, 1, 4, 1, Success);
		make_flix_episode!(&db, 1, 2, 4);
		assert_episode!(&db, 1, 2, 4, 2, Success);
		make_flix_episode!(&db, 1, 2, 5);
		assert_episode!(&db, 1, 2, 5, 2, Success);

		assert_season!(&db, 1, 1, 1, Watched);
		assert_season!(&db, 1, 1, 2, Watched);
		assert_season!(&db, 1, 1, 3, Unwatched);
		assert_season!(&db, 1, 2, 1, Watched);
		assert_season!(&db, 1, 2, 2, Unwatched);
		assert_season!(&db, 1, 2, 3, Unwatched);
		assert_season!(&db, 1, 3, 1, Unwatched);
		assert_season!(&db, 1, 3, 2, Unwatched);
		assert_season!(&db, 1, 3, 3, Unwatched);

		assert_show!(&db, 1, 1, Watched);
		assert_show!(&db, 1, 2, Unwatched);
		assert_show!(&db, 1, 3, Unwatched);
		assert_show!(&db, 2, 1, Unwatched);
		assert_show!(&db, 2, 2, Unwatched);
		assert_show!(&db, 2, 3, Unwatched);

		// Collection
		macro_rules! assert_collection {
			($db:expr, $id:literal, $uid:literal, Watched) => {
				assert_collection!(@find, $db, $id, $uid)
					.ok_or(())
					.expect("is none");
			};
			($db:expr, $id:literal, $uid:literal, Unwatched) => {
				assert_collection!(@find, $db, $id, $uid)
					.ok_or(())
					.expect_err("is some");
			};
			(@find, $db:expr, $id:literal, $uid:literal) => {
				super::collections::Entity::find()
					.filter(
						Condition::all()
							.add(super::collections::Column::Id.eq($id))
							.add(super::collections::Column::UserId.eq($uid)),
					)
					.one(&db)
					.await
					.expect("find.filter.one")
			};
		}

		have_collection!(&db, 1, 1, None);
		have_movie!(&db, 1, 1, Some(1));
		assert_movie!(&db, 1, 1, Success);
		assert_movie!(&db, 1, 2, Success);
		have_movie!(&db, 1, 2, Some(1));
		assert_movie!(&db, 2, 1, Success);
		assert_movie!(&db, 2, 2, Success);

		have_collection!(&db, 1, 2, Some(1));
		have_movie!(&db, 1, 3, Some(2));
		have_show!(&db, 1, 2, Some(2));
		have_season!(&db, 1, 2, 1);
		have_episode!(&db, 1, 2, 1, 1);
		assert_episode!(&db, 2, 1, 1, 1, Success);
		assert_movie!(&db, 3, 1, Success);
		have_movie!(&db, 1, 4, Some(2));
		assert_movie!(&db, 4, 1, Success);

		have_collection!(&db, 1, 3, Some(2));
		have_movie!(&db, 1, 5, Some(3));
		assert_movie!(&db, 5, 1, Success);
		have_movie!(&db, 1, 6, Some(3));
		assert_movie!(&db, 6, 1, Success);
		assert_movie!(&db, 6, 2, Success);

		have_collection!(&db, 1, 4, Some(3));
		have_movie!(&db, 1, 7, Some(4));
		assert_movie!(&db, 7, 1, Success);
		assert_movie!(&db, 7, 2, Success);
		have_movie!(&db, 1, 8, Some(4));
		assert_movie!(&db, 8, 1, Success);
		assert_movie!(&db, 8, 2, Success);

		assert_collection!(&db, 1, 1, Watched);
		assert_collection!(&db, 1, 2, Unwatched);
		assert_collection!(&db, 2, 1, Watched);
		assert_collection!(&db, 2, 2, Unwatched);
		assert_collection!(&db, 3, 1, Watched);
		assert_collection!(&db, 3, 2, Unwatched);
		assert_collection!(&db, 4, 1, Watched);
		assert_collection!(&db, 4, 2, Watched);
	}
}
