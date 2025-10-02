//! This module contains entities for storing media file information

pub mod libraries;

pub mod collections;

pub mod movies;

pub mod episodes;
pub mod seasons;
pub mod shows;

#[cfg(test)]
mod tests {
	use std::path::Path;

	use flix_model::id::{CollectionId, LibraryId, MovieId, ShowId};

	use sea_orm::ActiveModelTrait;
	use sea_orm::ActiveValue::{NotSet, Set};
	use sea_orm::sqlx::error::ErrorKind;
	use sea_orm_migration::MigratorTrait;

	use crate::migration::Migrator;
	use crate::tests::new_memory_db;

	use super::super::tests::get_error_kind;
	use super::super::tests::{
		make_flix_collection, make_flix_episode, make_flix_movie, make_flix_season, make_flix_show,
	};
	use super::super::tests::{noneable, notsettable};

	#[tokio::test]
	async fn test_inserts() {
		let db = new_memory_db().await;
		Migrator::up(&db, None).await.expect("up");

		// Libraries
		macro_rules! assert_library {
			($db:expr, $id:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_library!(@insert, $db, $id $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, LibraryId::from_raw($id));
				assert_eq!(model.directory, Path::new(concat!("/L/", $id)).to_owned().into());
			};
			($db:expr, $id:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_library!(@insert, $db, $id $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal $(; $($skip:ident),+)?) => {
				super::libraries::ActiveModel {
					id: notsettable!(id, LibraryId::from_raw($id) $(, $($skip),+)?),
					directory: notsettable!(directory, Path::new(concat!("/L/", $id)).to_owned().into() $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_library!(&db, 1, Success);
		assert_library!(&db, 1, UniqueViolation);
		assert_library!(&db, 2, Success);
		assert_library!(&db, 3, Success; id);
		assert_library!(&db, 4, NotNullViolation; directory);

		// Collections
		macro_rules! assert_collection {
			($db:expr, $id:literal, $pid:expr, $lid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_collection!(@insert, $db, $id, $pid, $lid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, CollectionId::from_raw($id));
				assert_eq!(model.parent, $pid);
				assert_eq!(model.slug, concat!("C/", $id).to_string());
				assert_eq!(model.library, LibraryId::from_raw($lid));
				assert_eq!(model.directory, Path::new(concat!("/C/", $id)).to_owned().into());
				assert_eq!(model.relative_poster_path, noneable!(relative_poster_path, concat!("C/Poster", $id).to_owned() $(, $($skip),+)?));
			};
			($db:expr, $id:literal, $pid:expr, $lid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_collection!(@insert, $db, $id, $pid, $lid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal, $pid:expr, $lid:literal $(; $($skip:ident),+)?) => {
				super::collections::ActiveModel {
					id: notsettable!(id, CollectionId::from_raw($id) $(, $($skip),+)?),
					parent: notsettable!(parent, $pid $(, $($skip),+)?),
					slug: notsettable!(slug, concat!("C/", $id).to_string() $(, $($skip),+)?),
					library: notsettable!(library, LibraryId::from_raw($lid) $(, $($skip),+)?),
					directory: notsettable!(directory, Path::new(concat!("/C/", $id)).to_owned().into() $(, $($skip),+)?),
					relative_poster_path: notsettable!(relative_poster_path, Some(concat!("C/Poster", $id).to_owned()) $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_collection!(&db, 1, None, 0, ForeignKeyViolation);
		assert_collection!(
			&db,
			1,
			Some(CollectionId::from_raw(0)),
			1,
			ForeignKeyViolation
		);
		assert_collection!(&db, 1, None, 1, ForeignKeyViolation);
		make_flix_collection!(&db, 1);
		make_flix_collection!(&db, 2);
		make_flix_collection!(&db, 3);
		make_flix_collection!(&db, 4);
		make_flix_collection!(&db, 8);

		assert_collection!(&db, 1, None, 1, Success);
		assert_collection!(&db, 1, None, 1, UniqueViolation);
		assert_collection!(&db, 2, None, 1, Success);
		assert_collection!(&db, 3, None, 1, Success; id);
		assert_collection!(&db, 4, None, 1, Success; parent);
		assert_collection!(&db, 5, None, 1, NotNullViolation; slug);
		assert_collection!(&db, 6, None, 1, NotNullViolation; library);
		assert_collection!(&db, 7, None, 1, NotNullViolation; directory);
		assert_collection!(&db, 8, None, 1, Success; relative_poster_path);

		// Movies
		macro_rules! assert_movie {
			($db:expr, $id:literal, $pid:expr, $lid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_movie!(@insert, $db, $id, $pid, $lid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, MovieId::from_raw($id));
				assert_eq!(model.parent, $pid);
				assert_eq!(model.slug, concat!("M/", $id).to_string());
				assert_eq!(model.library, LibraryId::from_raw($lid));
				assert_eq!(model.directory, Path::new(concat!("/M/", $id)).to_owned().into());
				assert_eq!(model.relative_media_path, concat!("M/Media", $id));
				assert_eq!(model.relative_poster_path, noneable!(relative_poster_path, concat!("M/Poster", $id).to_owned() $(, $($skip),+)?));
			};
			($db:expr, $id:literal, $pid:expr, $lid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_movie!(@insert, $db, $id, $pid, $lid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal, $pid:expr, $lid:literal $(; $($skip:ident),+)?) => {
				super::movies::ActiveModel {
					id: notsettable!(id, MovieId::from_raw($id) $(, $($skip),+)?),
					parent: notsettable!(parent, $pid $(, $($skip),+)?),
					slug: notsettable!(slug, concat!("M/", $id).to_string() $(, $($skip),+)?),
					library: notsettable!(library, LibraryId::from_raw($lid) $(, $($skip),+)?),
					directory: notsettable!(directory, Path::new(concat!("/M/", $id)).to_owned().into() $(, $($skip),+)?),
					relative_media_path: notsettable!(relative_media_path, concat!("M/Media", $id).to_owned() $(, $($skip),+)?),
					relative_poster_path: notsettable!(relative_poster_path, Some(concat!("M/Poster", $id).to_owned()) $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_movie!(&db, 1, None, 0, ForeignKeyViolation);
		assert_movie!(
			&db,
			1,
			Some(CollectionId::from_raw(0)),
			1,
			ForeignKeyViolation
		);
		assert_movie!(&db, 1, None, 1, ForeignKeyViolation);
		make_flix_movie!(&db, 1);
		make_flix_movie!(&db, 2);
		make_flix_movie!(&db, 3);
		make_flix_movie!(&db, 4);
		make_flix_movie!(&db, 9);

		assert_movie!(&db, 1, None, 1, Success);
		assert_movie!(&db, 1, None, 1, UniqueViolation);
		assert_movie!(&db, 2, None, 1, Success);
		assert_movie!(&db, 3, None, 1, Success; id);
		assert_movie!(&db, 4, None, 1, Success; parent);
		assert_movie!(&db, 5, None, 1, NotNullViolation; slug);
		assert_movie!(&db, 6, None, 1, NotNullViolation; library);
		assert_movie!(&db, 7, None, 1, NotNullViolation; directory);
		assert_movie!(&db, 8, None, 1, NotNullViolation; relative_media_path);
		assert_movie!(&db, 9, None, 1, Success; relative_poster_path);

		// Shows
		macro_rules! assert_show {
			($db:expr, $id:literal, $pid:expr, $lid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_show!(@insert, $db, $id, $pid, $lid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, ShowId::from_raw($id));
				assert_eq!(model.parent, $pid);
				assert_eq!(model.slug, concat!("S/", $id).to_string());
				assert_eq!(model.library, LibraryId::from_raw($lid));
				assert_eq!(model.directory, Path::new(concat!("/S/", $id)).to_owned().into());
				assert_eq!(model.relative_poster_path, noneable!(relative_poster_path, concat!("S/Poster", $id).to_owned() $(, $($skip),+)?));
			};
			($db:expr, $id:literal, $pid:expr, $lid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_show!(@insert, $db, $id, $pid, $lid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal, $pid:expr, $lid:literal $(; $($skip:ident),+)?) => {
				super::shows::ActiveModel {
					id: notsettable!(id, ShowId::from_raw($id) $(, $($skip),+)?),
					parent: notsettable!(parent, $pid $(, $($skip),+)?),
					slug: notsettable!(slug, concat!("S/", $id).to_string() $(, $($skip),+)?),
					library: notsettable!(library, LibraryId::from_raw($lid) $(, $($skip),+)?),
					directory: notsettable!(directory, Path::new(concat!("/S/", $id)).to_owned().into() $(, $($skip),+)?),
					relative_poster_path: notsettable!(relative_poster_path, Some(concat!("S/Poster", $id).to_owned()) $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_show!(&db, 1, None, 0, ForeignKeyViolation);
		assert_show!(
			&db,
			1,
			Some(CollectionId::from_raw(0)),
			1,
			ForeignKeyViolation
		);
		assert_show!(&db, 1, None, 1, ForeignKeyViolation);
		make_flix_show!(&db, 1);
		make_flix_show!(&db, 2);
		make_flix_show!(&db, 3);
		make_flix_show!(&db, 4);
		make_flix_show!(&db, 8);

		assert_show!(&db, 1, None, 1, Success);
		assert_show!(&db, 1, None, 1, UniqueViolation);
		assert_show!(&db, 2, None, 1, Success);
		assert_show!(&db, 3, None, 1, Success; id);
		assert_show!(&db, 4, None, 1, Success; parent);
		assert_show!(&db, 5, None, 1, NotNullViolation; slug);
		assert_show!(&db, 6, None, 1, NotNullViolation; library);
		assert_show!(&db, 7, None, 1, NotNullViolation; directory);
		assert_show!(&db, 8, None, 1, Success; relative_poster_path);

		// Seasons
		macro_rules! assert_season {
			($db:expr, $id:literal, $season:literal, $lid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_season!(@insert, $db, $id, $season, $lid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.show, ShowId::from_raw($id));
				assert_eq!(model.season, $season);
				assert_eq!(model.slug, concat!("S/S", $id).to_string());
				assert_eq!(model.library, LibraryId::from_raw($lid));
				assert_eq!(model.directory, Path::new(concat!("/S/S", $id)).to_owned().into());
				assert_eq!(model.relative_poster_path, noneable!(relative_poster_path, concat!("S/S/Poster", $id).to_owned() $(, $($skip),+)?));
			};
			($db:expr, $id:literal, $season:literal, $lid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_season!(@insert, $db, $id, $season, $lid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal, $season:literal, $lid:literal $(; $($skip:ident),+)?) => {
				super::seasons::ActiveModel {
					show: notsettable!(id, ShowId::from_raw($id) $(, $($skip),+)?),
					season: notsettable!(season, $season $(, $($skip),+)?),
					slug: notsettable!(slug, concat!("S/S", $id).to_string() $(, $($skip),+)?),
					library: notsettable!(library, LibraryId::from_raw($lid) $(, $($skip),+)?),
					directory: notsettable!(directory, Path::new(concat!("/S/S", $id)).to_owned().into() $(, $($skip),+)?),
					relative_poster_path: notsettable!(relative_poster_path, Some(concat!("S/S/Poster", $id).to_owned()) $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_season!(&db, 1, 1, 0, ForeignKeyViolation);
		assert_season!(&db, 1, 1, 1, ForeignKeyViolation);
		make_flix_season!(&db, 1, 1);
		make_flix_season!(&db, 1, 2);
		make_flix_season!(&db, 2, 1);
		make_flix_season!(&db, 3, 1);
		make_flix_season!(&db, 8, 1);

		assert_season!(&db, 1, 1, 1, Success);
		assert_season!(&db, 1, 2, 1, Success);
		assert_season!(&db, 1, 1, 1, UniqueViolation);
		assert_season!(&db, 2, 1, 1, Success);
		assert_season!(&db, 3, 1, 1, Success; show);
		assert_season!(&db, 4, 1, 1, NotNullViolation; season);
		assert_season!(&db, 5, 1, 1, NotNullViolation; slug);
		assert_season!(&db, 6, 1, 1, NotNullViolation; library);
		assert_season!(&db, 7, 1, 1, NotNullViolation; directory);
		assert_season!(&db, 8, 1, 1, Success; relative_poster_path);

		// Episodes
		macro_rules! assert_episode {
			($db:expr, $id:literal, $season:literal, $episode:literal, $lid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_episode!(@insert, $db, $id, $season, $episode, $lid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.show, ShowId::from_raw($id));
				assert_eq!(model.season, $season);
				assert_eq!(model.episode, $episode);
				assert_eq!(model.slug, concat!("S/S/E", $id).to_string());
				assert_eq!(model.library, LibraryId::from_raw($lid));
				assert_eq!(model.directory, Path::new(concat!("/S/S/E", $id)).to_owned().into());
				assert_eq!(model.relative_media_path, concat!("E/Media", $id));
				assert_eq!(model.relative_poster_path, noneable!(relative_poster_path, concat!("S/S/E/Poster", $id).to_owned() $(, $($skip),+)?));
			};
			($db:expr, $id:literal, $season:literal, $episode:literal, $lid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_episode!(@insert, $db, $id, $season, $episode, $lid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal, $season:literal, $episode:literal, $lid:literal $(; $($skip:ident),+)?) => {
				super::episodes::ActiveModel {
					show: notsettable!(id, ShowId::from_raw($id) $(, $($skip),+)?),
					season: notsettable!(season, $season $(, $($skip),+)?),
					episode: notsettable!(episode, $episode $(, $($skip),+)?),
					count: notsettable!(count, 0 $(, $($skip),+)?),
					slug: notsettable!(slug, concat!("S/S/E", $id).to_string() $(, $($skip),+)?),
					library: notsettable!(library, LibraryId::from_raw($lid) $(, $($skip),+)?),
					directory: notsettable!(directory, Path::new(concat!("/S/S/E", $id)).to_owned().into() $(, $($skip),+)?),
					relative_media_path: notsettable!(relative_media_path, concat!("E/Media", $id).to_owned() $(, $($skip),+)?),
					relative_poster_path: notsettable!(relative_poster_path, Some(concat!("S/S/E/Poster", $id).to_owned()) $(, $($skip),+)?),
				}.insert($db).await
			};
		}
		assert_episode!(&db, 1, 1, 1, 0, ForeignKeyViolation);
		assert_episode!(&db, 1, 1, 1, 1, ForeignKeyViolation);
		make_flix_episode!(&db, 1, 1, 1);
		make_flix_episode!(&db, 1, 1, 2);
		make_flix_episode!(&db, 2, 1, 1);
		make_flix_episode!(&db, 3, 1, 1);
		make_flix_show!(&db, 11);
		make_flix_season!(&db, 11, 1);
		make_flix_episode!(&db, 11, 1, 1);

		assert_episode!(&db, 1, 1, 1, 1, Success);
		assert_episode!(&db, 1, 1, 2, 1, Success);
		assert_episode!(&db, 1, 1, 1, 1, UniqueViolation);
		assert_episode!(&db, 2, 1, 1, 1, Success);
		assert_episode!(&db, 3, 1, 1, 1, Success; show);
		assert_episode!(&db, 4, 1, 1, 1, NotNullViolation; season);
		assert_episode!(&db, 5, 1, 1, 1, NotNullViolation; episode);
		assert_episode!(&db, 6, 1, 1, 1, NotNullViolation; count);
		assert_episode!(&db, 7, 1, 1, 1, NotNullViolation; slug);
		assert_episode!(&db, 8, 1, 1, 1, NotNullViolation; library);
		assert_episode!(&db, 9, 1, 1, 1, NotNullViolation; directory);
		assert_episode!(&db, 10, 1, 1, 1, NotNullViolation; relative_media_path);
		assert_episode!(&db, 11, 1, 1, 1, Success; relative_poster_path);
	}
}
