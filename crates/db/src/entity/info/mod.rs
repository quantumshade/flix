//! This module contains entities for storing media information such as
//! titles and overviews

pub mod collections;

pub mod movies;

pub mod episodes;
pub mod seasons;
pub mod shows;

#[cfg(test)]
mod tests {
	use flix_model::id::{CollectionId, MovieId, ShowId};

	use chrono::NaiveDate;
	use sea_orm::ActiveModelTrait;
	use sea_orm::ActiveValue::{NotSet, Set};
	use sea_orm::sqlx::error::ErrorKind;
	use sea_orm_migration::MigratorTrait;

	use crate::migration::Migrator;
	use crate::tests::new_memory_db;

	use super::super::tests::get_error_kind;
	use super::super::tests::notsettable;

	#[tokio::test]
	async fn test_inserts() {
		let db = new_memory_db().await;
		Migrator::up(&db, None).await.expect("up");

		// Collections
		macro_rules! assert_collection {
			($db:expr, $id:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_collection!(@insert, $db, $id $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, CollectionId::from_raw($id));
				assert_eq!(model.title, concat!("C", $id));
				assert_eq!(model.overview, concat!("Collection", " ", $id));
			};
			($db:expr, $id:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_collection!(@insert, $db, $id $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal $(; $($skip:ident),+)?) => {
				super::collections::ActiveModel {
					id: notsettable!(id, CollectionId::from_raw($id) $(, $($skip),+)?),
					title: notsettable!(title, concat!("C", $id).to_string() $(, $($skip),+)?),
					overview: notsettable!(overview, concat!("Collection", " ", $id).to_string() $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_collection!(&db, 1, Success);
		assert_collection!(&db, 1, UniqueViolation);
		assert_collection!(&db, 2, Success);
		assert_collection!(&db, 3, Success; id);
		assert_collection!(&db, 4, NotNullViolation; title);
		assert_collection!(&db, 5, NotNullViolation; overview);

		// Movies
		macro_rules! assert_movie {
			($db:expr, $id:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_movie!(@insert, $db, $id $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, MovieId::from_raw($id));
				assert_eq!(model.title, concat!("M", $id));
				assert_eq!(model.tagline, concat!("Watch Movie", " ", $id));
				assert_eq!(model.overview, concat!("Movie", " ", $id));
				assert_eq!(model.date, NaiveDate::from_yo_opt(2000, $id).expect("NaiveDate::from_yo_opt"));
			};
			($db:expr, $id:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_movie!(@insert, $db, $id $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal $(; $($skip:ident),+)?) => {
				super::movies::ActiveModel {
					id: notsettable!(id, MovieId::from_raw($id) $(, $($skip),+)?),
					title: notsettable!(title, concat!("M", $id).to_string() $(, $($skip),+)?),
					tagline: notsettable!(tagline, concat!("Watch Movie", " ", $id).to_string() $(, $($skip),+)?),
					overview: notsettable!(overview, concat!("Movie", " ", $id).to_string() $(, $($skip),+)?),
					date: notsettable!(date, NaiveDate::from_yo_opt(2000, $id).expect("NaiveDate::from_yo_opt") $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_movie!(&db, 1, Success);
		assert_movie!(&db, 1, UniqueViolation);
		assert_movie!(&db, 2, Success);
		assert_movie!(&db, 3, Success; id);
		assert_movie!(&db, 4, NotNullViolation; title);
		assert_movie!(&db, 5, NotNullViolation; tagline);
		assert_movie!(&db, 6, NotNullViolation; overview);
		assert_movie!(&db, 7, NotNullViolation; date);

		// Shows
		macro_rules! assert_show {
			($db:expr, $id:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_show!(@insert, $db, $id $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, ShowId::from_raw($id));
				assert_eq!(model.title, concat!("S", $id));
				assert_eq!(model.tagline, concat!("Watch Show", " ", $id));
				assert_eq!(model.overview, concat!("Show", " ", $id));
				assert_eq!(model.date, NaiveDate::from_yo_opt(2000, $id).expect("NaiveDate::from_yo_opt"));
			};
			($db:expr, $id:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_show!(@insert, $db, $id $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(
					get_error_kind(model).expect("get_error_kind"),
					ErrorKind::$error
				);
			};
			(@insert, $db:expr, $id:literal $(; $($skip:ident),+)?) => {
				super::shows::ActiveModel {
					id: notsettable!(id, ShowId::from_raw($id) $(, $($skip),+)?),
					title: notsettable!(title, concat!("S", $id).to_string() $(, $($skip),+)?),
					tagline: notsettable!(tagline, concat!("Watch Show", " ", $id).to_string() $(, $($skip),+)?),
					overview: notsettable!(overview, concat!("Show", " ", $id).to_string() $(, $($skip),+)?),
					date: notsettable!(date, NaiveDate::from_yo_opt(2000, $id).expect("NaiveDate::from_yo_opt") $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_show!(&db, 1, Success);
		assert_show!(&db, 1, UniqueViolation);
		assert_show!(&db, 2, Success);
		assert_show!(&db, 3, Success; id);
		assert_show!(&db, 4, NotNullViolation; title);
		assert_show!(&db, 5, NotNullViolation; tagline);
		assert_show!(&db, 6, NotNullViolation; overview);
		assert_show!(&db, 7, NotNullViolation; date);

		// Seasons
		macro_rules! assert_season {
			($db:expr, $show:literal, $season:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_season!(@insert, $db, $show, $season $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.show, ShowId::from_raw($show));
				assert_eq!(model.season, $season);
				assert_eq!(model.title, concat!("S", $show, "S", $season));
				assert_eq!(model.overview, concat!("Show", " ", $show, " ", "Season", " ", $season));
				assert_eq!(model.date, NaiveDate::from_yo_opt(2000, $show + $season).expect("NaiveDate::from_yo_opt"));
			};
			($db:expr, $show:literal, $season:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_season!(@insert, $db, $show, $season $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(
					get_error_kind(model).expect("get_error_kind"),
					ErrorKind::$error
				);
			};
			(@insert, $db:expr, $show:literal, $season:literal $(; $($skip:ident),+)?) => {
				super::seasons::ActiveModel {
					show: notsettable!(show, ShowId::from_raw($show) $(, $($skip),+)?),
					season: notsettable!(season, $season $(, $($skip),+)?),
					title: notsettable!(title, concat!("S", $show, "S", $season).to_string() $(, $($skip),+)?),
					overview: notsettable!(overview, concat!("Show", " ", $show, " ", "Season", " ", $season).to_string() $(, $($skip),+)?),
					date: notsettable!(date, NaiveDate::from_yo_opt(2000, $show + $season).expect("NaiveDate::from_yo_opt") $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_season!(&db, 1, 1, Success);
		assert_season!(&db, 1, 1, UniqueViolation);
		assert_season!(&db, 2, 1, Success);
		assert_season!(&db, 0, 1, ForeignKeyViolation);
		assert_season!(&db, 1, 2, Success);
		assert_season!(&db, 1, 3, NotNullViolation; show);
		assert_season!(&db, 1, 4, NotNullViolation; season);
		assert_season!(&db, 1, 5, NotNullViolation; title);
		assert_season!(&db, 1, 6, NotNullViolation; overview);
		assert_season!(&db, 1, 7, NotNullViolation; date);

		// Episodes
		macro_rules! assert_episode {
			($db:expr, $show:literal, $season:literal, $episode:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_episode!(@insert, $db, $show, $season, $episode $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.show, ShowId::from_raw($show));
				assert_eq!(model.season, $season);
				assert_eq!(model.episode, $episode);
				assert_eq!(model.title, concat!("S", $show, "S", $season, "E", $episode));
				assert_eq!(model.overview, concat!("Show", " ", $show, " ", "Season", " ", $season, " ", "Episode", " ", $episode));
				assert_eq!(model.date, NaiveDate::from_yo_opt(2000, $show + $season + $episode).expect("NaiveDate::from_yo_opt"));
			};
			($db:expr, $show:literal, $season:literal, $episode:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_episode!(@insert, $db, $show, $season, $episode $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(
					get_error_kind(model).expect("get_error_kind"),
					ErrorKind::$error
				);
			};
			(@insert, $db:expr, $show:literal, $season:literal, $episode:literal $(; $($skip:ident),+)?) => {
				super::episodes::ActiveModel {
					show: notsettable!(show, ShowId::from_raw($show) $(, $($skip),+)?),
					season: notsettable!(season, $season $(, $($skip),+)?),
					episode: notsettable!(episode, $episode $(, $($skip),+)?),
					title: notsettable!(title, concat!("S", $show, "S", $season, "E", $episode).to_string() $(, $($skip),+)?),
					overview: notsettable!(overview, concat!("Show", " ", $show, " ", "Season", " ", $season, " ", "Episode", " ", $episode).to_string() $(, $($skip),+)?),
					date: notsettable!(date, NaiveDate::from_yo_opt(2000, $show + $season + $episode).expect("NaiveDate::from_yo_opt") $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_episode!(&db, 1, 1, 1, Success);
		assert_episode!(&db, 1, 1, 1, UniqueViolation);
		assert_episode!(&db, 1, 2, 1, Success);
		assert_episode!(&db, 2, 1, 1, Success);
		assert_episode!(&db, 1, 0, 1, ForeignKeyViolation);
		assert_episode!(&db, 0, 1, 1, ForeignKeyViolation);
		assert_episode!(&db, 1, 1, 2, Success);
		assert_episode!(&db, 1, 1, 3, NotNullViolation; show);
		assert_episode!(&db, 1, 1, 4, NotNullViolation; season);
		assert_episode!(&db, 1, 1, 4, NotNullViolation; episode);
		assert_episode!(&db, 1, 1, 5, NotNullViolation; title);
		assert_episode!(&db, 1, 1, 6, NotNullViolation; overview);
		assert_episode!(&db, 1, 1, 7, NotNullViolation; date);
	}
}
