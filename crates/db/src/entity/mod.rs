//! Entity structs for interacting with the database.

#![expect(clippy::exhaustive_enums, reason = "reflects the database schema")]
#![expect(clippy::exhaustive_structs, reason = "reflects the database schema")]
#![expect(clippy::derive_partial_eq_without_eq, reason = "#[sea_orm::model]")]
#![expect(clippy::impl_trait_in_params, reason = "#[sea_orm::model]")]
#![expect(clippy::same_name_method, reason = "#[sea_orm::model]")]

pub mod content;
pub mod info;
#[cfg(feature = "tmdb")]
pub mod tmdb;
pub mod watched;

#[cfg(test)]
mod tests {
	use flix_model::numbers::{EpisodeNumber, SeasonNumber};
	#[cfg(feature = "tmdb")]
	use flix_tmdb::model::id::TmdbRepr as TmdbReprId;

	use sea_orm::ActiveValue::Set;
	use sea_orm::entity::prelude::*;
	use sea_orm::sqlx::error::ErrorKind;
	use seamantic::model::id::SeaOrmRepr as SeaOrmReprId;

	use crate::entity;
	use crate::entity::content::test::{
		make_content_collection, make_content_episode, make_content_library, make_content_movie,
		make_content_season, make_content_show,
	};
	#[cfg(feature = "tmdb")]
	use crate::entity::tmdb::test::{
		make_tmdb_collection, make_tmdb_episode, make_tmdb_movie, make_tmdb_season, make_tmdb_show,
	};
	use crate::entity::watched::test::{make_watched_episode, make_watched_movie};
	use crate::tests::new_initialized_memory_db;

	#[derive(Debug)]
	pub(crate) enum ErrorKindNot {
		Runtime,
		Sqlx,
		Database,
	}

	/// Extract the error kind from a [`DbErr`].
	///
	/// # Errors
	/// If the [`DbErr`] internals diverge from [`SqlxError::Database`].
	pub(crate) fn get_error_kind(error: DbErr) -> Result<ErrorKind, ErrorKindNot> {
		let (DbErr::Conn(runtime_err) | DbErr::Exec(runtime_err) | DbErr::Query(runtime_err)) =
			error
		else {
			return Err(ErrorKindNot::Runtime);
		};

		let RuntimeErr::SqlxError(sqlx_err) = runtime_err else {
			return Err(ErrorKindNot::Sqlx);
		};

		let SqlxError::Database(database_err) = sqlx_err.as_ref() else {
			return Err(ErrorKindNot::Database);
		};

		Ok(database_err.kind())
	}

	/// Helper macro for writing tests for `ActiveModel` structs where
	/// toggling [`sea_orm::ActiveValue`] is needed.
	macro_rules! notsettable {
		($field:ident, $value:expr $(, $($skip:ident),+)?) => {
			if notsettable!(@skip, $field $(, $($skip),+)?) {
				NotSet
			} else {
				Set($value)
			}
		};
		(@skip, $field:ident $(, $skip:ident),*) => {
			false $(|| stringify!($field) == stringify!($skip))*
		};
	}
	pub(super) use notsettable;

	/// Helper macro for writing tests for `ActiveModel` structs where
	/// toggling [`sea_orm::ActiveValue`] is needed.
	macro_rules! noneable {
			($field:ident, $value:expr $(, $($skip:ident),+)?) => {
				if noneable!(@skip, $field $(, $($skip),+)?) {
					None
				} else {
					Some($value)
				}
			};
			(@skip, $field:ident $(, $skip:ident),*) => {
				false $(|| stringify!($field) == stringify!($skip))*
			};
		}
	pub(super) use noneable;

	struct FlixId {
		collection: SeaOrmReprId,
		movie: SeaOrmReprId,
		show: SeaOrmReprId,
		season: u32,
		episode: u32,
	}

	#[cfg(feature = "tmdb")]
	struct TmdbId {
		collection: TmdbReprId,
		movie: TmdbReprId,
		show: TmdbReprId,
		season: u32,
		episode: u32,
	}

	struct ContentId {
		library: SeaOrmReprId,
	}

	struct WatchId {
		user: SeaOrmReprId,
	}

	struct DbId {
		flix: FlixId,
		#[cfg(feature = "tmdb")]
		tmdb: TmdbId,
		content: ContentId,
		watch: WatchId,
	}

	impl Default for DbId {
		fn default() -> Self {
			Self {
				flix: FlixId {
					collection: 1,
					movie: 2,
					show: 3,
					season: 4,
					episode: 5,
				},
				#[cfg(feature = "tmdb")]
				tmdb: TmdbId {
					collection: 6,
					movie: 7,
					show: 8,
					season: 9,
					episode: 10,
				},
				content: ContentId { library: 11 },
				watch: WatchId { user: 12 },
			}
		}
	}

	/// Initialize a database with data to be deleted.
	///
	/// # Panics
	/// If any database operation fails, since this is only for testing.
	#[expect(clippy::default_numeric_fallback, reason = "unit test")]
	async fn initialize_deletion_test_database(id: &DbId) -> DatabaseConnection {
		let db = new_initialized_memory_db().await;

		// Populate the database
		make_content_library!(&db, id.content.library);
		make_content_collection!(&db, id.content.library, id.flix.collection, None);
		make_content_movie!(
			&db,
			id.content.library,
			id.flix.movie,
			Some(id.flix.collection)
		);
		make_content_show!(
			&db,
			id.content.library,
			id.flix.show,
			Some(id.flix.collection)
		);
		make_content_season!(&db, id.content.library, id.flix.show, id.flix.season);
		make_content_episode!(
			&db,
			id.content.library,
			id.flix.show,
			id.flix.season,
			id.flix.episode
		);

		#[cfg(feature = "tmdb")]
		make_tmdb_collection!(&db, id.tmdb.collection, id.flix.collection);
		#[cfg(feature = "tmdb")]
		make_tmdb_movie!(&db, id.tmdb.movie, id.flix.movie);
		#[cfg(feature = "tmdb")]
		make_tmdb_show!(&db, id.tmdb.show, id.flix.show);
		#[cfg(feature = "tmdb")]
		make_tmdb_season!(
			&db,
			id.tmdb.show,
			id.tmdb.season,
			id.flix.show,
			id.flix.season
		);
		#[cfg(feature = "tmdb")]
		make_tmdb_episode!(
			&db,
			id.tmdb.show,
			id.tmdb.season,
			id.tmdb.episode,
			id.flix.show,
			id.flix.season,
			id.flix.episode
		);

		make_watched_movie!(&db, id.flix.movie, id.watch.user);
		make_watched_episode!(
			&db,
			id.flix.show,
			id.flix.season,
			id.flix.episode,
			id.watch.user
		);

		// Check that everything has one entry
		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);

		db
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_info_collection() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::info::collections::Entity::delete_by_id(seamantic::model::id::Id::from_raw(
			id.flix.collection,
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(0),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(0),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(0),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_info_movie() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::info::movies::Entity::delete_by_id(seamantic::model::id::Id::from_raw(
			id.flix.movie,
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(0), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(0), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(0),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(0),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_info_show() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::info::shows::Entity::delete_by_id(seamantic::model::id::Id::from_raw(
			id.flix.show,
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(0), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(0),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(0), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(0),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(0),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_info_season() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::info::seasons::Entity::delete_by_id((
			seamantic::model::id::Id::from_raw(id.flix.show),
			SeasonNumber::new(id.flix.season),
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(0),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(0),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(0),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_info_episodes() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::info::episodes::Entity::delete_by_id((
			seamantic::model::id::Id::from_raw(id.flix.show),
			SeasonNumber::new(id.flix.season),
			EpisodeNumber::new(id.flix.episode),
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(0),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[cfg(feature = "tmdb")]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_tmdb_collection() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::tmdb::collections::Entity::delete_by_id(flix_tmdb::model::id::Id::from_raw(
			id.tmdb.collection,
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(0),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[cfg(feature = "tmdb")]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_tmdb_movie() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::tmdb::movies::Entity::delete_by_id(flix_tmdb::model::id::Id::from_raw(
			id.tmdb.movie,
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(0), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[cfg(feature = "tmdb")]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_tmdb_show() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::tmdb::shows::Entity::delete_by_id(flix_tmdb::model::id::Id::from_raw(
			id.tmdb.show,
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(0), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(0),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(0),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[cfg(feature = "tmdb")]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_tmdb_season() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::tmdb::seasons::Entity::delete_by_id((
			flix_tmdb::model::id::Id::from_raw(id.tmdb.show),
			SeasonNumber::new(id.tmdb.season),
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(0),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(0),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[cfg(feature = "tmdb")]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_tmdb_episode() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::tmdb::episodes::Entity::delete_by_id((
			flix_tmdb::model::id::Id::from_raw(id.tmdb.show),
			SeasonNumber::new(id.tmdb.season),
			EpisodeNumber::new(id.tmdb.episode),
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(0),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_content_library() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::content::libraries::Entity::delete_by_id(seamantic::model::id::Id::from_raw(
			id.content.library,
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(0),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(0),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_content_collection() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::content::collections::Entity::delete_by_id(seamantic::model::id::Id::from_raw(
			id.flix.collection,
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(0),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_content_movie() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::content::movies::Entity::delete_by_id(seamantic::model::id::Id::from_raw(
			id.flix.movie,
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(0),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_content_show() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::content::shows::Entity::delete_by_id(seamantic::model::id::Id::from_raw(
			id.flix.show,
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_content_season() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::content::seasons::Entity::delete_by_id((
			seamantic::model::id::Id::from_raw(id.flix.show),
			SeasonNumber::new(id.flix.season),
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_content_episode() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::content::episodes::Entity::delete_by_id((
			seamantic::model::id::Id::from_raw(id.flix.show),
			SeasonNumber::new(id.flix.season),
			EpisodeNumber::new(id.flix.episode),
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_watched_movie() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::watched::movies::Entity::delete_by_id((
			seamantic::model::id::Id::from_raw(id.flix.movie),
			id.watch.user,
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(0),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}

	#[cfg(not(miri))]
	#[tokio::test]
	#[expect(clippy::missing_panics_doc, reason = "unit test")]
	async fn delete_watched_episode() {
		let id = DbId::default();
		let db = initialize_deletion_test_database(&id).await;

		_ = entity::watched::episodes::Entity::delete_by_id((
			seamantic::model::id::Id::from_raw(id.flix.show),
			SeasonNumber::new(id.flix.season),
			EpisodeNumber::new(id.flix.episode),
			id.watch.user,
		))
		.exec(&db)
		.await
		.expect("Entity::delete_by_id");

		assert_eq!(
			Ok(1),
			entity::info::collections::Entity::find().count(&db).await
		);
		assert_eq!(Ok(1), entity::info::movies::Entity::find().count(&db).await);
		assert_eq!(Ok(1), entity::info::shows::Entity::find().count(&db).await);
		assert_eq!(
			Ok(1),
			entity::info::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::info::episodes::Entity::find().count(&db).await
		);

		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::collections::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::movies::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(Ok(1), entity::tmdb::shows::Entity::find().count(&db).await);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::seasons::Entity::find().count(&db).await
		);
		#[cfg(feature = "tmdb")]
		assert_eq!(
			Ok(1),
			entity::tmdb::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::content::libraries::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::collections::Entity::find()
				.count(&db)
				.await
		);
		assert_eq!(
			Ok(1),
			entity::content::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::shows::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::seasons::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(1),
			entity::content::episodes::Entity::find().count(&db).await
		);

		assert_eq!(
			Ok(1),
			entity::watched::movies::Entity::find().count(&db).await
		);
		assert_eq!(
			Ok(0),
			entity::watched::episodes::Entity::find().count(&db).await
		);
	}
}
