//! This module contains entities for storing media file information

/// Library entity
pub mod libraries {
	use flix_model::id::LibraryId;

	use seamantic::model::path::PathBytes;

	use chrono::{DateTime, Utc};
	use sea_orm::entity::prelude::*;

	/// The database representation of a library media folder
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_libraries")]
	pub struct Model {
		/// The library's ID
		#[sea_orm(column_type = "Integer", primary_key, nullable, auto_increment = false)]
		pub id: LibraryId,
		/// The library's directory
		pub directory: PathBytes,
		/// The library's last scan data
		pub last_scan: Option<DateTime<Utc>>,

		/// Collections that are part of this library
		#[sea_orm(has_many, on_update = "Cascade", on_delete = "Cascade")]
		pub collections: HasMany<super::collections::Entity>,
		/// Movies that are part of this library
		#[sea_orm(has_many, on_update = "Cascade", on_delete = "Cascade")]
		pub movies: HasMany<super::movies::Entity>,
		/// Shows that are part of this library
		#[sea_orm(has_many, on_update = "Cascade", on_delete = "Cascade")]
		pub shows: HasMany<super::shows::Entity>,
		/// Seasons that are part of this library
		#[sea_orm(has_many, on_update = "Cascade", on_delete = "Cascade")]
		pub seasons: HasMany<super::seasons::Entity>,
		/// Episodes that are part of this library
		#[sea_orm(has_many, on_update = "Cascade", on_delete = "Cascade")]
		pub episodes: HasMany<super::episodes::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Collection entity
pub mod collections {
	use flix_model::id::{CollectionId, LibraryId};

	use seamantic::model::path::PathBytes;

	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a collection media folder
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_collections")]
	pub struct Model {
		/// The collection's ID
		#[sea_orm(column_type = "Integer", primary_key, nullable, auto_increment = false)]
		pub id: CollectionId,
		/// The collection's parent
		#[sea_orm(indexed)]
		pub parent_id: Option<CollectionId>,
		/// The collection's slug
		#[sea_orm(unique)]
		pub slug: String,
		/// The collection's library ID
		pub library_id: LibraryId,
		/// The collection's directory
		pub directory: PathBytes,
		/// The collection's poster path
		pub relative_poster_path: Option<String>,

		/// This collection's parent
		#[sea_orm(self_ref, relation_enum = "Parent", from = "parent_id", to = "id")]
		pub parent: HasOne<Entity>,
		/// The library this collection belongs to
		#[sea_orm(belongs_to, from = "library_id", to = "id")]
		pub library: HasOne<super::libraries::Entity>,
		/// The info for this collection
		#[sea_orm(belongs_to, relation_enum = "Info", from = "id", to = "id")]
		pub info: HasOne<entity::info::collections::Entity>,
		/// The watched info for this collection
		#[sea_orm(has_many, relation_enum = "Watched", from = "id", to = "id")]
		pub watched: HasMany<entity::watched::collections::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Movie entity
pub mod movies {
	use flix_model::id::{CollectionId, LibraryId, MovieId};

	use seamantic::model::path::PathBytes;

	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a movie media folder
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_movies")]
	pub struct Model {
		/// The movie's ID
		#[sea_orm(column_type = "Integer", primary_key, nullable, auto_increment = false)]
		pub id: MovieId,
		/// The movie's parent
		#[sea_orm(indexed)]
		pub parent_id: Option<CollectionId>,
		/// The movie's slug
		#[sea_orm(unique)]
		pub slug: String,
		/// The movie's library
		pub library_id: LibraryId,
		/// The movie's directory
		pub directory: PathBytes,
		/// The movie's media path
		pub relative_media_path: String,
		/// The movie's poster path
		pub relative_poster_path: Option<String>,

		/// This movie's parent
		#[sea_orm(belongs_to, from = "parent_id", to = "id")]
		pub parent: HasOne<super::collections::Entity>,
		/// The library this movie belongs to
		#[sea_orm(belongs_to, from = "library_id", to = "id")]
		pub library: HasOne<super::libraries::Entity>,
		/// The info for this movie
		#[sea_orm(belongs_to, relation_enum = "Info", from = "id", to = "id")]
		pub info: HasOne<entity::info::movies::Entity>,
		/// The watched info for this movie
		#[sea_orm(has_many, relation_enum = "Watched", from = "id", to = "id")]
		pub watched: HasMany<entity::watched::movies::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Show entity
pub mod shows {
	use flix_model::id::{CollectionId, LibraryId, ShowId};

	use seamantic::model::path::PathBytes;

	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a show media folder
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_shows")]
	pub struct Model {
		/// The show's ID
		#[sea_orm(column_type = "Integer", primary_key, nullable, auto_increment = false)]
		pub id: ShowId,
		/// The show's parent
		#[sea_orm(indexed)]
		pub parent_id: Option<CollectionId>,
		/// The show's slug
		#[sea_orm(unique)]
		pub slug: String,
		/// The show's library
		pub library_id: LibraryId,
		/// The show's directory
		pub directory: PathBytes,
		/// The show's poster path
		pub relative_poster_path: Option<String>,

		/// This show's parent
		#[sea_orm(belongs_to, from = "parent_id", to = "id")]
		pub parent: HasOne<super::collections::Entity>,
		/// The library this show belongs to
		#[sea_orm(belongs_to, from = "library_id", to = "id")]
		pub library: HasOne<super::libraries::Entity>,
		/// The info for this show
		#[sea_orm(belongs_to, relation_enum = "Info", from = "id", to = "id")]
		pub info: HasOne<entity::info::shows::Entity>,
		/// The watched info for this show
		#[sea_orm(has_many, relation_enum = "Watched", from = "id", to = "id")]
		pub watched: HasMany<entity::watched::shows::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Season entity
pub mod seasons {
	use flix_model::id::{LibraryId, ShowId};
	use flix_model::numbers::SeasonNumber;

	use seamantic::model::path::PathBytes;

	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a season media folder
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_seasons")]
	pub struct Model {
		/// The season's show's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub show_id: ShowId,
		/// The season's number
		#[sea_orm(primary_key, auto_increment = false)]
		pub season_number: SeasonNumber,
		/// The season's slug
		#[sea_orm(unique)]
		pub slug: String,
		/// The season's library
		pub library_id: LibraryId,
		/// The season's directory
		pub directory: PathBytes,
		/// The season's poster path
		pub relative_poster_path: Option<String>,

		/// The library this season belongs to
		#[sea_orm(belongs_to, from = "library_id", to = "id")]
		pub library: HasOne<super::libraries::Entity>,
		/// The info for this season
		#[sea_orm(
			belongs_to,
			relation_enum = "Info",
			from = "(show_id, season_number)",
			to = "(show_id, season_number)"
		)]
		pub info: HasOne<entity::info::seasons::Entity>,
		/// The watched info for this season
		#[sea_orm(
			has_many,
			relation_enum = "Watched",
			from = "(show_id, season_number)",
			to = "(show_id, season_number)"
		)]
		pub watched: HasMany<entity::watched::seasons::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Episode entity
pub mod episodes {
	use flix_model::id::{LibraryId, ShowId};
	use flix_model::numbers::{EpisodeNumber, SeasonNumber};

	use seamantic::model::path::PathBytes;

	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a episode media folder
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_episodes")]
	pub struct Model {
		/// The episode's show's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub show_id: ShowId,
		/// The episode's season's number
		#[sea_orm(primary_key, auto_increment = false)]
		pub season_number: SeasonNumber,
		/// The episode's number
		#[sea_orm(primary_key, auto_increment = false)]
		pub episode_number: EpisodeNumber,
		/// The number of additional contained episodes
		pub count: u8,
		/// The episode's slug
		#[sea_orm(unique)]
		pub slug: String,
		/// The episode's library
		pub library_id: LibraryId,
		/// The episode's directory
		pub directory: PathBytes,
		/// The episode's media path
		pub relative_media_path: String,
		/// The episode's poster path
		pub relative_poster_path: Option<String>,

		/// The library this episode belongs to
		#[sea_orm(belongs_to, from = "library_id", to = "id")]
		pub library: HasOne<super::libraries::Entity>,
		/// The info for this episode
		#[sea_orm(
			belongs_to,
			relation_enum = "Info",
			from = "(show_id, season_number, episode_number)",
			to = "(show_id, season_number, episode_number)"
		)]
		pub info: HasOne<entity::info::episodes::Entity>,
		/// The watched info for this episode
		#[sea_orm(
			has_many,
			relation_enum = "Watched",
			from = "(show_id, season_number, episode_number)",
			to = "(show_id, season_number, episode_number)"
		)]
		pub watched: HasMany<entity::watched::episodes::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Macros for creating content entities
#[cfg(test)]
pub mod test {
	macro_rules! make_content_library {
		($db:expr, $id:literal) => {
			$crate::entity::content::libraries::ActiveModel {
				id: Set(::flix_model::id::LibraryId::from_raw($id)),
				directory: Set(::std::path::PathBuf::new().into()),
				last_scan: Set(None),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_content_library;

	macro_rules! make_content_collection {
		($db:expr, $lid:literal, $id:literal, $pid:expr) => {
			$crate::entity::info::test::make_info_collection!($db, $id);
			$crate::entity::content::collections::ActiveModel {
				id: Set(::flix_model::id::CollectionId::from_raw($id)),
				parent_id: Set($pid.map(::flix_model::id::CollectionId::from_raw)),
				slug: Set(concat!("C ", $id).to_string()),
				library_id: Set(::flix_model::id::LibraryId::from_raw($lid)),
				directory: Set(::std::path::PathBuf::new().into()),
				relative_poster_path: Set(::core::option::Option::None),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_content_collection;

	macro_rules! make_content_movie {
		($db:expr, $lid:literal, $id:literal, $pid:expr) => {
			$crate::entity::info::test::make_info_movie!($db, $id);
			$crate::entity::content::movies::ActiveModel {
				id: Set(::flix_model::id::MovieId::from_raw($id)),
				parent_id: Set($pid.map(::flix_model::id::CollectionId::from_raw)),
				slug: Set(concat!("< ", $id).to_string()),
				library_id: Set(::flix_model::id::LibraryId::from_raw($lid)),
				directory: Set(::std::path::PathBuf::new().into()),
				relative_media_path: Set(::std::string::String::new()),
				relative_poster_path: Set(::core::option::Option::None),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_content_movie;

	macro_rules! make_content_show {
		($db:expr, $lid:literal, $id:literal, $pid:expr) => {
			$crate::entity::info::test::make_info_show!($db, $id);
			$crate::entity::content::shows::ActiveModel {
				id: Set(::flix_model::id::ShowId::from_raw($id)),
				parent_id: Set($pid.map(::flix_model::id::CollectionId::from_raw)),
				slug: Set(concat!("S ", $id).to_string()),
				library_id: Set(::flix_model::id::LibraryId::from_raw($lid)),
				directory: Set(::std::path::PathBuf::new().into()),
				relative_poster_path: Set(::core::option::Option::None),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_content_show;

	macro_rules! make_content_season {
		($db:expr, $lid:literal, $show:literal, $season:literal) => {
			$crate::entity::info::test::make_info_season!($db, $show, $season);
			$crate::entity::content::seasons::ActiveModel {
				show_id: Set(::flix_model::id::ShowId::from_raw($show)),
				season_number: Set(::flix_model::numbers::SeasonNumber::new($season)),
				slug: Set(concat!("SS ", $show, $season).to_string()),
				library_id: Set(::flix_model::id::LibraryId::from_raw($lid)),
				directory: Set(::std::path::PathBuf::new().into()),
				relative_poster_path: Set(::core::option::Option::None),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_content_season;

	macro_rules! make_content_episode {
		($db:expr, $lid:literal, $show:literal, $season:literal, $episode:literal) => {
			make_content_episode!(@make, $db, $lid, $show, $season, $episode, 0);
		};
		($db:expr, $lid:literal, $show:literal, $season:literal, $episode:literal, >1) => {
			make_content_episode!(@make, $db, $lid, $show, $season, $episode, 1);
		};
		(@make, $db:expr, $lid:literal, $show:literal, $season:literal, $episode:literal, $count:literal) => {
			$crate::entity::info::test::make_info_episode!($db, $show, $season, $episode);
			$crate::entity::content::episodes::ActiveModel {
				show_id: Set(::flix_model::id::ShowId::from_raw($show)),
				season_number: Set(::flix_model::numbers::SeasonNumber::new($season)),
				episode_number: Set(::flix_model::numbers::EpisodeNumber::new($episode)),
				count: Set($count),
				slug: Set(concat!("SSE ", $show, $season, $episode).to_string()),
				library_id: Set(::flix_model::id::LibraryId::from_raw($lid)),
				directory: Set(::std::path::PathBuf::new().into()),
				relative_media_path: Set(::std::string::String::new()),
				relative_poster_path: Set(::core::option::Option::None),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_content_episode;
}

#[cfg(test)]
mod tests {
	use std::path::Path;

	use flix_model::id::{CollectionId, LibraryId, MovieId, ShowId};

	use chrono::NaiveDate;
	use sea_orm::ActiveValue::{NotSet, Set};
	use sea_orm::entity::prelude::*;
	use sea_orm::sqlx::error::ErrorKind;

	use crate::entity::content::test::{
		make_content_collection, make_content_episode, make_content_library, make_content_movie,
		make_content_season, make_content_show,
	};
	use crate::entity::info::test::{
		make_info_collection, make_info_episode, make_info_movie, make_info_season, make_info_show,
	};
	use crate::tests::new_initialized_memory_db;

	use super::super::tests::get_error_kind;
	use super::super::tests::{noneable, notsettable};

	#[tokio::test]
	async fn use_test_macros() {
		let db = new_initialized_memory_db().await;

		make_content_library!(&db, 1);
		make_content_collection!(&db, 1, 1, None);
		make_content_movie!(&db, 1, 1, None);
		make_content_show!(&db, 1, 1, None);
		make_content_season!(&db, 1, 1, 1);
		make_content_episode!(&db, 1, 1, 1, 1);
	}

	#[tokio::test]
	async fn test_round_trip_libraries() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_library {
			($db:expr, $id:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_library!(@insert, $db, $id $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, LibraryId::from_raw($id));
				assert_eq!(model.directory, Path::new(concat!("L Directory ", $id)).to_owned().into());
				assert_eq!(model.last_scan, noneable!(last_scan, NaiveDate::from_yo_opt($id, 1).expect("from_yo_opt").and_hms_opt(0, 0, 0).expect("and_hms_opt").and_utc() $(, $($skip),+)?));
			};
			($db:expr, $id:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_library!(@insert, $db, $id $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal $(; $($skip:ident),+)?) => {
				super::libraries::ActiveModel {
					id: notsettable!(id, LibraryId::from_raw($id) $(, $($skip),+)?),
					directory: notsettable!(directory, Path::new(concat!("L Directory ", $id)).to_owned().into() $(, $($skip),+)?),
					last_scan: notsettable!(last_scan, Some(NaiveDate::from_yo_opt($id, 1).expect("from_yo_opt").and_hms_opt(0, 0, 0).expect("and_hms_opt").and_utc()) $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		assert_library!(&db, 1, Success);
		assert_library!(&db, 1, UniqueViolation);
		assert_library!(&db, 2, Success);
		assert_library!(&db, 3, Success; id);
		assert_library!(&db, 4, NotNullViolation; directory);
		assert_library!(&db, 5, Success; last_scan);
	}

	#[tokio::test]
	async fn test_round_trip_collections() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_collection {
			($db:expr, $id:literal, $pid:expr, $lid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_collection!(@insert, $db, $id, $pid, $lid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, CollectionId::from_raw($id));
				assert_eq!(model.parent_id, $pid.map(CollectionId::from_raw));
				assert_eq!(model.slug, concat!("C Slug ", $id).to_string());
				assert_eq!(model.library_id, LibraryId::from_raw($lid));
				assert_eq!(model.directory, Path::new(concat!("C Directory ", $id)).to_owned().into());
				assert_eq!(model.relative_poster_path, noneable!(relative_poster_path, concat!("C Poster ", $id).to_owned() $(, $($skip),+)?));
			};
			($db:expr, $id:literal, $pid:expr, $lid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_collection!(@insert, $db, $id, $pid, $lid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal, $pid:expr, $lid:literal $(; $($skip:ident),+)?) => {
				super::collections::ActiveModel {
					id: notsettable!(id, CollectionId::from_raw($id) $(, $($skip),+)?),
					parent_id: notsettable!(parent_id, $pid.map(CollectionId::from_raw) $(, $($skip),+)?),
					slug: notsettable!(slug, concat!("C Slug ", $id).to_string() $(, $($skip),+)?),
					library_id: notsettable!(library_id, LibraryId::from_raw($lid) $(, $($skip),+)?),
					directory: notsettable!(directory, Path::new(concat!("C Directory ", $id)).to_owned().into() $(, $($skip),+)?),
					relative_poster_path: notsettable!(relative_poster_path, Some(concat!("C Poster ", $id).to_owned()) $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		make_content_library!(&db, 1);
		assert_collection!(&db, 1, None, 1, ForeignKeyViolation);
		make_info_collection!(&db, 1);
		assert_collection!(&db, 1, None, 1, Success);
		make_info_collection!(&db, 2);
		assert_collection!(&db, 2, None, 2, ForeignKeyViolation);
		make_content_library!(&db, 2);
		assert_collection!(&db, 2, None, 2, Success);

		assert_collection!(&db, 1, None, 1, UniqueViolation);
		make_info_collection!(&db, 3);
		make_info_collection!(&db, 4);
		make_info_collection!(&db, 5);
		make_info_collection!(&db, 6);
		make_info_collection!(&db, 7);
		make_info_collection!(&db, 8);
		assert_collection!(&db, 3, None, 1, Success; id);
		assert_collection!(&db, 4, None, 1, Success; parent_id);
		assert_collection!(&db, 5, None, 1, NotNullViolation; slug);
		assert_collection!(&db, 6, None, 1, NotNullViolation; library_id);
		assert_collection!(&db, 7, None, 1, NotNullViolation; directory);
		assert_collection!(&db, 8, None, 1, Success; relative_poster_path);
	}

	#[tokio::test]
	async fn test_round_trip_movies() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_movie {
			($db:expr, $id:literal, $pid:expr, $lid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_movie!(@insert, $db, $id, $pid, $lid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, MovieId::from_raw($id));
				assert_eq!(model.parent_id, $pid.map(CollectionId::from_raw));
				assert_eq!(model.slug, concat!("M Slug ", $id).to_string());
				assert_eq!(model.library_id, LibraryId::from_raw($lid));
				assert_eq!(model.directory, Path::new(concat!("M Directory ", $id)).to_owned().into());
				assert_eq!(model.relative_media_path, concat!("M Media ", $id));
				assert_eq!(model.relative_poster_path, noneable!(relative_poster_path, concat!("M Poster ", $id).to_owned() $(, $($skip),+)?));
			};
			($db:expr, $id:literal, $pid:expr, $lid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_movie!(@insert, $db, $id, $pid, $lid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal, $pid:expr, $lid:literal $(; $($skip:ident),+)?) => {
				super::movies::ActiveModel {
					id: notsettable!(id, MovieId::from_raw($id) $(, $($skip),+)?),
					parent_id: notsettable!(parent_id, $pid.map(CollectionId::from_raw) $(, $($skip),+)?),
					slug: notsettable!(slug, concat!("M Slug ", $id).to_string() $(, $($skip),+)?),
					library_id: notsettable!(library_id, LibraryId::from_raw($lid) $(, $($skip),+)?),
					directory: notsettable!(directory, Path::new(concat!("M Directory ", $id)).to_owned().into() $(, $($skip),+)?),
					relative_media_path: notsettable!(relative_media_path, concat!("M Media ", $id).to_owned() $(, $($skip),+)?),
					relative_poster_path: notsettable!(relative_poster_path, Some(concat!("M Poster ", $id).to_owned()) $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		make_content_library!(&db, 1);
		assert_movie!(&db, 1, None, 1, ForeignKeyViolation);
		make_info_movie!(&db, 1);
		assert_movie!(&db, 1, Some(1), 1, ForeignKeyViolation);
		make_content_collection!(&db, 1, 1, None);
		assert_movie!(&db, 1, Some(1), 1, Success);
		assert_movie!(&db, 2, None, 2, ForeignKeyViolation);
		make_info_movie!(&db, 2);
		assert_movie!(&db, 2, None, 2, ForeignKeyViolation);
		make_content_library!(&db, 2);
		assert_movie!(&db, 2, None, 2, Success);

		assert_movie!(&db, 1, None, 1, UniqueViolation);
		make_info_movie!(&db, 3);
		make_info_movie!(&db, 4);
		make_info_movie!(&db, 5);
		make_info_movie!(&db, 6);
		make_info_movie!(&db, 7);
		make_info_movie!(&db, 8);
		make_info_movie!(&db, 9);
		assert_movie!(&db, 3, None, 1, Success; id);
		assert_movie!(&db, 4, None, 1, Success; parent_id);
		assert_movie!(&db, 5, None, 1, NotNullViolation; slug);
		assert_movie!(&db, 6, None, 1, NotNullViolation; library_id);
		assert_movie!(&db, 7, None, 1, NotNullViolation; directory);
		assert_movie!(&db, 8, None, 1, NotNullViolation; relative_media_path);
		assert_movie!(&db, 9, None, 1, Success; relative_poster_path);
	}

	#[tokio::test]
	async fn test_round_trip_shows() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_show {
			($db:expr, $id:literal, $pid:expr, $lid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_show!(@insert, $db, $id, $pid, $lid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, ShowId::from_raw($id));
				assert_eq!(model.parent_id, $pid.map(CollectionId::from_raw));
				assert_eq!(model.slug, concat!("S Slug ", $id).to_string());
				assert_eq!(model.library_id, LibraryId::from_raw($lid));
				assert_eq!(model.directory, Path::new(concat!("S Directory ", $id)).to_owned().into());
				assert_eq!(model.relative_poster_path, noneable!(relative_poster_path, concat!("S Poster ", $id).to_owned() $(, $($skip),+)?));
			};
			($db:expr, $id:literal, $pid:expr, $lid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_show!(@insert, $db, $id, $pid, $lid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal, $pid:expr, $lid:literal $(; $($skip:ident),+)?) => {
				super::shows::ActiveModel {
					id: notsettable!(id, ShowId::from_raw($id) $(, $($skip),+)?),
					parent_id: notsettable!(parent_id, $pid.map(CollectionId::from_raw) $(, $($skip),+)?),
					slug: notsettable!(slug, concat!("S Slug ", $id).to_string() $(, $($skip),+)?),
					library_id: notsettable!(library_id, LibraryId::from_raw($lid) $(, $($skip),+)?),
					directory: notsettable!(directory, Path::new(concat!("S Directory ", $id)).to_owned().into() $(, $($skip),+)?),
					relative_poster_path: notsettable!(relative_poster_path, Some(concat!("S Poster ", $id).to_owned()) $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		make_content_library!(&db, 1);
		assert_show!(&db, 1, None, 1, ForeignKeyViolation);
		make_info_show!(&db, 1);
		assert_show!(&db, 1, Some(1), 1, ForeignKeyViolation);
		make_content_collection!(&db, 1, 1, None);
		assert_show!(&db, 1, Some(1), 1, Success);
		assert_show!(&db, 2, None, 2, ForeignKeyViolation);
		make_info_show!(&db, 2);
		assert_show!(&db, 2, None, 2, ForeignKeyViolation);
		make_content_library!(&db, 2);
		assert_show!(&db, 2, None, 2, Success);

		assert_show!(&db, 1, None, 1, UniqueViolation);
		make_info_show!(&db, 3);
		make_info_show!(&db, 4);
		make_info_show!(&db, 5);
		make_info_show!(&db, 6);
		make_info_show!(&db, 7);
		make_info_show!(&db, 8);
		assert_show!(&db, 3, None, 1, Success; id);
		assert_show!(&db, 4, None, 1, Success; parent_id);
		assert_show!(&db, 5, None, 1, NotNullViolation; slug);
		assert_show!(&db, 6, None, 1, NotNullViolation; library_id);
		assert_show!(&db, 7, None, 1, NotNullViolation; directory);
		assert_show!(&db, 8, None, 1, Success; relative_poster_path);
	}

	#[tokio::test]
	async fn test_round_trip_seasons() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_season {
			($db:expr, $id:literal, $season:literal, $lid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_season!(@insert, $db, $id, $season, $lid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.show_id, ShowId::from_raw($id));
				assert_eq!(model.season_number, ::flix_model::numbers::SeasonNumber::new($season));
				assert_eq!(model.slug, concat!("SS Slug ", $id, ",", $season).to_string());
				assert_eq!(model.library_id, LibraryId::from_raw($lid));
				assert_eq!(model.directory, Path::new(concat!("SS Directory ", $id, ",", $season)).to_owned().into());
				assert_eq!(model.relative_poster_path, noneable!(relative_poster_path, concat!("SS Poster ", $id, ",", $season).to_owned() $(, $($skip),+)?));
			};
			($db:expr, $id:literal, $season:literal, $lid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_season!(@insert, $db, $id, $season, $lid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal, $season:literal, $lid:literal $(; $($skip:ident),+)?) => {
				super::seasons::ActiveModel {
					show_id: notsettable!(show_id, ShowId::from_raw($id) $(, $($skip),+)?),
					season_number: notsettable!(season_number, ::flix_model::numbers::SeasonNumber::new($season) $(, $($skip),+)?),
					slug: notsettable!(slug, concat!("SS Slug ", $id, ",", $season).to_string() $(, $($skip),+)?),
					library_id: notsettable!(library_id, LibraryId::from_raw($lid) $(, $($skip),+)?),
					directory: notsettable!(directory, Path::new(concat!("SS Directory ", $id, ",", $season)).to_owned().into() $(, $($skip),+)?),
					relative_poster_path: notsettable!(relative_poster_path, Some(concat!("SS Poster ", $id, ",", $season).to_owned()) $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		make_content_library!(&db, 1);
		make_info_show!(&db, 1);
		assert_season!(&db, 1, 1, 1, ForeignKeyViolation);
		make_info_season!(&db, 1, 1);
		assert_season!(&db, 1, 1, 1, Success);

		assert_season!(&db, 1, 1, 1, UniqueViolation);
		make_info_season!(&db, 1, 3);
		make_info_season!(&db, 1, 4);
		make_info_season!(&db, 1, 5);
		make_info_season!(&db, 1, 6);
		make_info_season!(&db, 1, 7);
		make_info_season!(&db, 1, 8);
		assert_season!(&db, 1, 3, 1, NotNullViolation; show_id);
		assert_season!(&db, 1, 4, 1, NotNullViolation; season_number);
		assert_season!(&db, 1, 5, 1, NotNullViolation; slug);
		assert_season!(&db, 1, 6, 1, NotNullViolation; library_id);
		assert_season!(&db, 1, 7, 1, NotNullViolation; directory);
		assert_season!(&db, 1, 8, 1, Success; relative_poster_path);
	}

	#[tokio::test]
	async fn test_round_trip_episodes() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_episode {
			($db:expr, $id:literal, $season:literal, $episode:literal, $lid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_episode!(@insert, $db, $id, $season, $episode, $lid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.show_id, ShowId::from_raw($id));
				assert_eq!(model.season_number, ::flix_model::numbers::SeasonNumber::new($season));
				assert_eq!(model.episode_number, ::flix_model::numbers::EpisodeNumber::new($episode));
				assert_eq!(model.slug, concat!("SS Slug ", $id, ",", $season, $episode).to_string());
				assert_eq!(model.library_id, LibraryId::from_raw($lid));
				assert_eq!(model.directory, Path::new(concat!("SS Directory ", $id, ",", $season, $episode)).to_owned().into());
				assert_eq!(model.relative_media_path, concat!("SS Media ", $id, ",", $season, $episode));
				assert_eq!(model.relative_poster_path, noneable!(relative_poster_path, concat!("SS Poster ", $id, ",", $season, $episode).to_owned() $(, $($skip),+)?));
			};
			($db:expr, $id:literal, $season:literal, $episode:literal, $lid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_episode!(@insert, $db, $id, $season, $episode, $lid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal, $season:literal, $episode:literal, $lid:literal $(; $($skip:ident),+)?) => {
				super::episodes::ActiveModel {
					show_id: notsettable!(show_id, ShowId::from_raw($id) $(, $($skip),+)?),
					season_number: notsettable!(season_number, ::flix_model::numbers::SeasonNumber::new($season) $(, $($skip),+)?),
					episode_number: notsettable!(episode_number, ::flix_model::numbers::EpisodeNumber::new($episode) $(, $($skip),+)?),
					count: notsettable!(count, 0 $(, $($skip),+)?),
					slug: notsettable!(slug, concat!("SS Slug ", $id, ",", $season, $episode).to_string() $(, $($skip),+)?),
					library_id: notsettable!(library_id, LibraryId::from_raw($lid) $(, $($skip),+)?),
					directory: notsettable!(directory, Path::new(concat!("SS Directory ", $id, ",", $season, $episode)).to_owned().into() $(, $($skip),+)?),
					relative_media_path: notsettable!(relative_media_path, concat!("SS Media ", $id, ",", $season, $episode).to_owned() $(, $($skip),+)?),
					relative_poster_path: notsettable!(relative_poster_path, Some(concat!("SS Poster ", $id, ",", $season, $episode).to_owned()) $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		make_content_library!(&db, 1);
		make_info_show!(&db, 1);
		make_info_season!(&db, 1, 1);
		assert_episode!(&db, 1, 1, 1, 1, ForeignKeyViolation);
		make_info_episode!(&db, 1, 1, 1);
		assert_episode!(&db, 1, 1, 1, 1, Success);

		assert_episode!(&db, 1, 1, 1, 1, UniqueViolation);
		make_info_episode!(&db, 1, 1, 3);
		make_info_episode!(&db, 1, 1, 4);
		make_info_episode!(&db, 1, 1, 5);
		make_info_episode!(&db, 1, 1, 6);
		make_info_episode!(&db, 1, 1, 7);
		make_info_episode!(&db, 1, 1, 8);
		make_info_episode!(&db, 1, 1, 9);
		make_info_episode!(&db, 1, 1, 10);
		assert_episode!(&db, 1, 1, 3, 1, NotNullViolation; show_id);
		assert_episode!(&db, 1, 1, 4, 1, NotNullViolation; season_number);
		assert_episode!(&db, 1, 1, 5, 1, NotNullViolation; episode_number);
		assert_episode!(&db, 1, 1, 6, 1, NotNullViolation; slug);
		assert_episode!(&db, 1, 1, 7, 1, NotNullViolation; library_id);
		assert_episode!(&db, 1, 1, 8, 1, NotNullViolation; directory);
		assert_episode!(&db, 1, 1, 9, 1, NotNullViolation; relative_media_path);
		assert_episode!(&db, 1, 1, 10, 1, Success; relative_poster_path);
	}
}
