//! This module contains entities for storing dynamic data from TMDB

/// Collection entity
pub mod collections {
	use flix_model::id::CollectionId as FlixId;
	use flix_tmdb::model::id::CollectionId;

	use chrono::NaiveDate;
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a tmdb collection
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_tmdb_collections")]
	pub struct Model {
		/// The collection's TMDB ID
		#[sea_orm(column_type = "Integer", primary_key, nullable, auto_increment = false)]
		pub tmdb_id: CollectionId,
		/// The collection's ID
		#[sea_orm(unique)]
		pub flix_id: FlixId,
		/// The date of the last update
		pub last_update: NaiveDate,
		/// The number of movies in the collection
		pub movie_count: u16,

		/// Movies that are in this collection
		#[sea_orm(has_many, on_update = "Cascade", on_delete = "Cascade")]
		pub movies: HasMany<super::movies::Entity>,
		/// The info for this collection
		#[sea_orm(belongs_to, from = "flix_id", to = "id")]
		pub info: HasOne<entity::info::collections::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Movie entity
pub mod movies {
	use flix_model::id::MovieId as FlixId;
	use flix_tmdb::model::id::{CollectionId, MovieId};

	use seamantic::model::duration::Seconds;

	use chrono::NaiveDate;
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a tmdb movie
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_tmdb_movies")]
	pub struct Model {
		/// The movie's TMDB ID
		#[sea_orm(column_type = "Integer", primary_key, nullable, auto_increment = false)]
		pub tmdb_id: MovieId,
		/// The movie's ID
		#[sea_orm(unique)]
		pub flix_id: FlixId,
		/// The date of the last update
		pub last_update: NaiveDate,
		/// The movie's runtime in seconds
		pub runtime: Seconds,
		/// The TMDB ID of the collection this movie belongs to
		#[sea_orm(indexed)]
		pub collection_id: Option<CollectionId>,

		/// The info for this collection
		#[sea_orm(belongs_to, from = "collection_id", to = "tmdb_id")]
		pub collection: HasOne<super::collections::Entity>,
		/// The info for this movie
		#[sea_orm(belongs_to, from = "flix_id", to = "id")]
		pub info: HasOne<entity::info::movies::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Show entity
pub mod shows {
	use flix_model::id::ShowId as FlixId;
	use flix_tmdb::model::id::ShowId;

	use chrono::NaiveDate;
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a tmdb show
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_tmdb_shows")]
	pub struct Model {
		/// The show's TMDB ID
		#[sea_orm(column_type = "Integer", primary_key, nullable, auto_increment = false)]
		pub tmdb_id: ShowId,
		/// The show's ID
		#[sea_orm(unique)]
		pub flix_id: FlixId,
		/// The movie's runtime in seconds
		pub last_update: NaiveDate,
		/// The number of seasons the show has
		pub number_of_seasons: u32,

		/// Seasons that are part of this show
		#[sea_orm(has_many, on_update = "Cascade", on_delete = "Cascade")]
		pub seasons: HasMany<super::seasons::Entity>,
		/// Episodes that are part of this show
		#[sea_orm(has_many, on_update = "Cascade", on_delete = "Cascade")]
		pub episodes: HasMany<super::episodes::Entity>,
		/// The info for this show
		#[sea_orm(belongs_to, from = "flix_id", to = "id")]
		pub info: HasOne<entity::info::shows::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Season entity
pub mod seasons {
	use flix_model::id::ShowId as FlixId;
	use flix_model::numbers::SeasonNumber;
	use flix_tmdb::model::id::ShowId;

	use chrono::NaiveDate;
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a tmdb season
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_tmdb_seasons")]
	pub struct Model {
		/// The season's show's TMDB ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub tmdb_show: ShowId,
		/// The season's TMDB season number
		#[sea_orm(primary_key, auto_increment = false)]
		pub tmdb_season: SeasonNumber,
		/// The season's show's ID
		#[sea_orm(unique_key = "flix")]
		pub flix_show: FlixId,
		/// The season's number
		#[sea_orm(unique_key = "flix")]
		pub flix_season: SeasonNumber,
		/// The date of the last update
		pub last_update: NaiveDate,

		/// The show this season belongs to
		#[sea_orm(belongs_to, from = "tmdb_show", to = "tmdb_id")]
		pub show: HasOne<super::shows::Entity>,
		/// The info for this season
		#[sea_orm(
			belongs_to,
			from = "(flix_show, flix_season)",
			to = "(show_id, season_number)"
		)]
		pub info: HasOne<entity::info::seasons::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Season entity
pub mod episodes {
	use flix_model::id::ShowId as FlixId;
	use flix_model::numbers::{EpisodeNumber, SeasonNumber};
	use flix_tmdb::model::id::ShowId;
	use seamantic::model::duration::Seconds;

	use chrono::NaiveDate;
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a tmdb episode
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_tmdb_episodes")]
	pub struct Model {
		/// The episode's show's TMDB ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub tmdb_show: ShowId,
		/// The episode's season's TMDB season number
		#[sea_orm(primary_key, auto_increment = false)]
		pub tmdb_season: SeasonNumber,
		/// The episode's TMDB episode number
		#[sea_orm(primary_key, auto_increment = false)]
		pub tmdb_episode: EpisodeNumber,
		/// The episode's show's ID
		#[sea_orm(unique_key = "flix")]
		pub flix_show: FlixId,
		/// The episode's season's number
		#[sea_orm(unique_key = "flix")]
		pub flix_season: SeasonNumber,
		/// The episode's number
		#[sea_orm(unique_key = "flix")]
		pub flix_episode: EpisodeNumber,
		/// The date of the last update
		pub last_update: NaiveDate,
		/// The episode's runtime in seconds
		pub runtime: Seconds,

		/// The show this episode belongs to
		#[sea_orm(belongs_to, from = "tmdb_show", to = "tmdb_id")]
		pub show: HasOne<super::shows::Entity>,
		/// The season this episode belongs to
		#[sea_orm(
			belongs_to,
			from = "(tmdb_show, tmdb_season)",
			to = "(tmdb_show, tmdb_season)"
		)]
		pub season: HasOne<super::seasons::Entity>,
		/// The info for this episode
		#[sea_orm(
			belongs_to,
			from = "(flix_show, flix_season, flix_episode)",
			to = "(show_id, season_number, episode_number)"
		)]
		pub info: HasOne<entity::info::episodes::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Macros for creating tmdb entities
#[cfg(test)]
pub mod test {
	macro_rules! make_tmdb_collection {
		($db:expr, $id:literal, $flix_id:literal) => {
			$crate::entity::tmdb::collections::ActiveModel {
				tmdb_id: Set(::flix_tmdb::model::id::CollectionId::from_raw($id)),
				flix_id: Set(::flix_model::id::CollectionId::from_raw($flix_id)),
				last_update: Set(::chrono::NaiveDate::from_yo_opt(1, 1).expect("from_yo_opt")),
				movie_count: Set(::core::default::Default::default()),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_tmdb_collection;

	macro_rules! make_tmdb_movie {
		($db:expr, $id:literal, $flix_id:literal) => {
			$crate::entity::tmdb::movies::ActiveModel {
				tmdb_id: Set(::flix_tmdb::model::id::MovieId::from_raw($id)),
				flix_id: Set(::flix_model::id::MovieId::from_raw($flix_id)),
				last_update: Set(::chrono::NaiveDate::from_yo_opt(1, 1).expect("from_yo_opt")),
				runtime: Set(::core::default::Default::default()),
				collection_id: Set(None),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_tmdb_movie;

	macro_rules! make_tmdb_show {
		($db:expr, $id:literal, $flix_id:literal) => {
			$crate::entity::tmdb::shows::ActiveModel {
				tmdb_id: Set(::flix_tmdb::model::id::ShowId::from_raw($id)),
				flix_id: Set(::flix_model::id::ShowId::from_raw($flix_id)),
				last_update: Set(::chrono::NaiveDate::from_yo_opt(1, 1).expect("from_yo_opt")),
				number_of_seasons: Set(::core::default::Default::default()),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_tmdb_show;

	macro_rules! make_tmdb_season {
		($db:expr, $show:literal, $season:literal, $flix_show:literal, $flix_season:literal) => {
			$crate::entity::tmdb::seasons::ActiveModel {
				tmdb_show: Set(::flix_tmdb::model::id::ShowId::from_raw($show)),
				tmdb_season: Set($season),
				flix_show: Set(::flix_model::id::ShowId::from_raw($flix_show)),
				flix_season: Set($flix_season),
				last_update: Set(::chrono::NaiveDate::from_yo_opt(1, 1).expect("from_yo_opt")),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_tmdb_season;

	macro_rules! make_tmdb_episode {
		($db:expr, $show:literal, $season:literal, $episode:literal, $flix_show:literal, $flix_season:literal, $flix_episode:literal) => {
			$crate::entity::tmdb::episodes::ActiveModel {
				tmdb_show: Set(::flix_tmdb::model::id::ShowId::from_raw($show)),
				tmdb_season: Set($season),
				tmdb_episode: Set($episode),
				flix_show: Set(::flix_model::id::ShowId::from_raw($flix_show)),
				flix_season: Set($flix_season),
				flix_episode: Set($flix_episode),
				last_update: Set(::chrono::NaiveDate::from_yo_opt(1, 1).expect("from_yo_opt")),
				runtime: Set(::core::default::Default::default()),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_tmdb_episode;
}

#[cfg(test)]
mod tests {
	use core::time::Duration;

	use flix_model::id::{CollectionId, MovieId, ShowId};
	use flix_tmdb::model::id::{
		CollectionId as TmdbCollectionId, MovieId as TmdbMovieId, ShowId as TmdbShowId,
	};

	use chrono::NaiveDate;
	use sea_orm::ActiveValue::{NotSet, Set};
	use sea_orm::entity::prelude::*;
	use sea_orm::sqlx::error::ErrorKind;

	use crate::entity::info::test::{
		make_info_collection, make_info_episode, make_info_movie, make_info_season, make_info_show,
	};
	use crate::tests::new_initialized_memory_db;

	use super::super::tests::get_error_kind;
	use super::super::tests::notsettable;
	use super::test::{
		make_tmdb_collection, make_tmdb_episode, make_tmdb_movie, make_tmdb_season, make_tmdb_show,
	};

	#[tokio::test]
	async fn use_test_macros() {
		let db = new_initialized_memory_db().await;

		make_info_collection!(&db, 1);
		make_info_movie!(&db, 1);
		make_info_show!(&db, 1);
		make_info_season!(&db, 1, 1);
		make_info_episode!(&db, 1, 1, 1);

		make_tmdb_collection!(&db, 1, 1);
		make_tmdb_movie!(&db, 1, 1);
		make_tmdb_show!(&db, 1, 1);
		make_tmdb_season!(&db, 1, 1, 1, 1);
		make_tmdb_episode!(&db, 1, 1, 1, 1, 1, 1);
	}

	#[tokio::test]
	async fn test_round_trip_collections() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_collection {
			($db:expr, $id:literal, $tid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_collection!(@insert, $db, $id, $tid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.tmdb_id, TmdbCollectionId::from_raw($tid));
				assert_eq!(model.flix_id, CollectionId::from_raw($id));
				assert_eq!(model.last_update, NaiveDate::from_yo_opt($id, 1).expect("from_yo_opt"));
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
					last_update: notsettable!(last_update, NaiveDate::from_yo_opt($id, 1).expect("from_yo_opt") $(, $($skip),+)?),
					movie_count: notsettable!(movie_count, $id $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		assert_collection!(&db, 1, 1, ForeignKeyViolation);
		make_info_collection!(&db, 1);
		assert_collection!(&db, 1, 1, Success);
		assert_collection!(&db, 1, 1, UniqueViolation);

		assert_collection!(&db, 1, 2, UniqueViolation);
		assert_collection!(&db, 2, 1, UniqueViolation);
		make_info_collection!(&db, 2);
		assert_collection!(&db, 2, 2, Success);

		make_info_collection!(&db, 3);
		assert_collection!(&db, 3, 3, Success; tmdb_id);
		assert_collection!(&db, 4, 4, NotNullViolation; flix_id);
		assert_collection!(&db, 5, 5, NotNullViolation; last_update);
		assert_collection!(&db, 6, 6, NotNullViolation; movie_count);
	}

	#[tokio::test]
	async fn test_round_trip_movies() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_movie {
			($db:expr, $id:literal, $tid:literal, $cid:expr, Success $(; $($skip:ident),+)?) => {
				let model = assert_movie!(@insert, $db, $id, $tid, $cid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.tmdb_id, TmdbMovieId::from_raw($tid));
				assert_eq!(model.flix_id, MovieId::from_raw($id));
				assert_eq!(model.last_update, NaiveDate::from_yo_opt($id, 1).expect("from_yo_opt"));
				assert_eq!(model.runtime, Duration::from_secs($tid).into());
				assert_eq!(model.collection_id, $cid.map(TmdbCollectionId::from_raw));
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
					last_update: notsettable!(last_update, NaiveDate::from_yo_opt($id, 1).expect("from_yo_opt") $(, $($skip),+)?),
					runtime: notsettable!(runtime, Duration::from_secs($tid).into() $(, $($skip),+)?),
					collection_id: notsettable!(collection_id, $cid.map(TmdbCollectionId::from_raw) $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		assert_movie!(&db, 1, 1, None, ForeignKeyViolation);
		make_info_movie!(&db, 1);
		assert_movie!(&db, 1, 1, None, Success);
		assert_movie!(&db, 1, 1, None, UniqueViolation);

		make_info_movie!(&db, 2);
		assert_movie!(&db, 2, 2, Some(2), ForeignKeyViolation);
		make_info_collection!(&db, 2);
		make_tmdb_collection!(&db, 2, 2);
		assert_movie!(&db, 2, 2, Some(2), Success);
		assert_movie!(&db, 1, 2, None, UniqueViolation);
		assert_movie!(&db, 2, 1, None, UniqueViolation);

		make_info_movie!(&db, 3);
		assert_movie!(&db, 3, 3, None, Success; tmdb_id);
		assert_movie!(&db, 4, 4, None, NotNullViolation; flix_id);
		assert_movie!(&db, 5, 5, None, NotNullViolation; last_update);
		assert_movie!(&db, 6, 6, None, NotNullViolation; runtime);
		assert_movie!(&db, 7, 7, None, ForeignKeyViolation; collection_id);
	}

	#[tokio::test]
	async fn test_round_trip_shows() {
		let db = new_initialized_memory_db().await;

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
		make_info_show!(&db, 1);
		assert_show!(&db, 1, 1, Success);
		assert_show!(&db, 1, 1, UniqueViolation);

		assert_show!(&db, 1, 2, UniqueViolation);
		assert_show!(&db, 2, 1, UniqueViolation);
		make_info_show!(&db, 2);
		assert_show!(&db, 2, 2, Success);

		make_info_show!(&db, 3);
		assert_show!(&db, 3, 3, Success; tmdb_id);
		assert_show!(&db, 4, 4, NotNullViolation; flix_id);
		assert_show!(&db, 5, 5, NotNullViolation; last_update);
		assert_show!(&db, 6, 6, NotNullViolation; number_of_seasons);
	}

	#[tokio::test]
	async fn test_round_trip_seasons() {
		let db = new_initialized_memory_db().await;

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

		make_info_show!(&db, 1);
		make_tmdb_show!(&db, 1, 1);

		assert_season!(&db, 1, 1, 1, 1, ForeignKeyViolation);
		make_info_season!(&db, 1, 1);
		assert_season!(&db, 1, 1, 1, 1, Success);

		assert_season!(&db, 1, 1, 1, 1, UniqueViolation);
		assert_season!(&db, 1, 1, 2, 1, UniqueViolation);
		assert_season!(&db, 2, 1, 1, 1, UniqueViolation);
		make_info_season!(&db, 1, 2);
		assert_season!(&db, 1, 2, 1, 2, Success);

		assert_season!(&db, 1, 3, 1, 3, NotNullViolation; tmdb_show);
		assert_season!(&db, 1, 4, 1, 4, NotNullViolation; tmdb_season);
		assert_season!(&db, 1, 5, 1, 5, NotNullViolation; flix_show);
		assert_season!(&db, 1, 6, 1, 6, NotNullViolation; flix_season);
		assert_season!(&db, 1, 7, 1, 7, NotNullViolation; last_update);
	}

	#[tokio::test]
	async fn test_round_trip_episodes() {
		let db = new_initialized_memory_db().await;

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

		make_info_show!(&db, 1);
		make_info_season!(&db, 1, 1);
		make_tmdb_show!(&db, 1, 1);
		make_tmdb_season!(&db, 1, 1, 1, 1);

		assert_episode!(&db, 1, 1, 1, 1, 1, 1, ForeignKeyViolation);
		make_info_episode!(&db, 1, 1, 1);
		assert_episode!(&db, 1, 1, 1, 1, 1, 1, Success);

		assert_episode!(&db, 1, 1, 1, 1, 1, 1, UniqueViolation);
		assert_episode!(&db, 1, 1, 1, 1, 2, 1, UniqueViolation);
		assert_episode!(&db, 1, 1, 1, 2, 1, 1, UniqueViolation);
		assert_episode!(&db, 1, 2, 1, 1, 1, 1, UniqueViolation);
		assert_episode!(&db, 2, 1, 1, 1, 1, 1, UniqueViolation);
		make_info_episode!(&db, 1, 1, 2);
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
