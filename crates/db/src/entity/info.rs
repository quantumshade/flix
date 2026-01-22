//! This module contains entities for storing media information such as
//! titles and overviews

/// Collection entity
pub mod collections {
	use flix_model::id::CollectionId;

	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a flix collection
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_info_collections")]
	pub struct Model {
		/// The collection's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub id: CollectionId,
		/// The collection's title
		pub title: String,
		/// The collection's overview
		pub overview: String,

		/// The sortable title
		#[sea_orm(indexed)]
		pub sort_title: String,
		/// The filesystem-safe slug
		#[sea_orm(indexed, unique)]
		pub fs_slug: String,
		/// The url-safe slug
		#[sea_orm(indexed, unique)]
		pub web_slug: String,

		/// Potential content for this collection
		#[sea_orm(has_one)]
		pub content: HasOne<entity::content::collections::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Movie entity
pub mod movies {
	use flix_model::id::MovieId;

	use chrono::NaiveDate;
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a flix movie
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_info_movies")]
	pub struct Model {
		/// The movie's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub id: MovieId,
		/// The movie's title
		pub title: String,
		/// The movie's tagline
		pub tagline: String,
		/// The movie's overview
		pub overview: String,
		/// The movie's release date
		#[sea_orm(indexed)]
		pub date: NaiveDate,

		/// The sortable title
		#[sea_orm(indexed)]
		pub sort_title: String,
		/// The filesystem-safe slug
		#[sea_orm(indexed, unique)]
		pub fs_slug: String,
		/// The url-safe slug
		#[sea_orm(indexed, unique)]
		pub web_slug: String,

		/// Potential content for this movie
		#[sea_orm(has_one)]
		pub content: HasOne<entity::content::movies::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Show entity
pub mod shows {
	use flix_model::id::ShowId;

	use chrono::NaiveDate;
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a flix show
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_info_shows")]
	pub struct Model {
		/// The show's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub id: ShowId,
		/// The show's title
		pub title: String,
		/// The show's tagline
		pub tagline: String,
		/// The show's overview
		pub overview: String,
		/// The show's air date
		#[sea_orm(indexed)]
		pub date: NaiveDate,

		/// The sortable title
		#[sea_orm(indexed)]
		pub sort_title: String,
		/// The filesystem-safe slug
		#[sea_orm(indexed, unique)]
		pub fs_slug: String,
		/// The url-safe slug
		#[sea_orm(indexed, unique)]
		pub web_slug: String,

		/// Seasons that are part of this show
		#[sea_orm(has_many)]
		pub seasons: HasMany<super::seasons::Entity>,
		/// Episodes that are part of this show
		#[sea_orm(has_many)]
		pub episodes: HasMany<super::episodes::Entity>,

		/// Potential content for this show
		#[sea_orm(has_one)]
		pub content: HasOne<entity::content::shows::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Season entity
pub mod seasons {
	use flix_model::id::ShowId;
	use flix_model::numbers::SeasonNumber;

	use chrono::NaiveDate;
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a flix season
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_info_seasons")]
	pub struct Model {
		/// The season's show's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub show_id: ShowId,
		/// The season's number
		#[sea_orm(primary_key, auto_increment = false)]
		pub season_number: SeasonNumber,
		/// The season's title
		pub title: String,
		/// The season's overview
		pub overview: String,
		/// The season's air date
		#[sea_orm(indexed)]
		pub date: NaiveDate,

		/// The show this season belongs to
		#[sea_orm(
			belongs_to,
			from = "show_id",
			to = "id",
			on_update = "Cascade",
			on_delete = "Cascade"
		)]
		pub show: HasOne<super::shows::Entity>,
		/// Episodes that are part of this season
		#[sea_orm(has_many)]
		pub episodes: HasMany<super::episodes::Entity>,

		/// Potential content for this season
		#[sea_orm(has_one)]
		pub content: HasOne<entity::content::seasons::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Episode entity
pub mod episodes {
	use flix_model::id::ShowId;
	use flix_model::numbers::{EpisodeNumber, SeasonNumber};

	use chrono::NaiveDate;
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a flix episode
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_info_episodes")]
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
		/// The episode's title
		pub title: String,
		/// The episode's overview
		pub overview: String,
		/// The episode's air date
		#[sea_orm(indexed)]
		pub date: NaiveDate,

		/// The show this episode belongs to
		#[sea_orm(
			belongs_to,
			from = "show_id",
			to = "id",
			on_update = "Cascade",
			on_delete = "Cascade"
		)]
		pub show: HasOne<super::shows::Entity>,
		/// The season this episode belongs to
		#[sea_orm(
			belongs_to,
			from = "(show_id, season_number)",
			to = "(show_id, season_number)",
			on_update = "Cascade",
			on_delete = "Cascade"
		)]
		pub season: HasOne<super::seasons::Entity>,

		/// Potential content for this episode
		#[sea_orm(has_one)]
		pub content: HasOne<entity::content::episodes::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Macros for creating info entities
#[cfg(test)]
pub mod test {
	macro_rules! make_info_collection {
		($db:expr, $id:expr) => {
			$crate::entity::info::collections::ActiveModel {
				id: Set(::flix_model::id::CollectionId::from_raw($id)),
				title: Set(::std::string::String::new()),
				overview: Set(::std::string::String::new()),
				sort_title: Set(::std::string::String::new()),
				fs_slug: Set(format!("C FS {}", $id)),
				web_slug: Set(format!("C Web {}", $id)),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_info_collection;

	macro_rules! make_info_movie {
		($db:expr, $id:expr) => {
			$crate::entity::info::movies::ActiveModel {
				id: Set(::flix_model::id::MovieId::from_raw($id)),
				title: Set(::std::string::String::new()),
				tagline: Set(::std::string::String::new()),
				overview: Set(::std::string::String::new()),
				date: Set(::chrono::NaiveDate::from_yo_opt(1, 1).expect("from_yo_opt")),
				sort_title: Set(::std::string::String::new()),
				fs_slug: Set(format!("M FS {}", $id)),
				web_slug: Set(format!("M Web {}", $id)),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_info_movie;

	macro_rules! make_info_show {
		($db:expr, $id:expr) => {
			$crate::entity::info::shows::ActiveModel {
				id: Set(::flix_model::id::ShowId::from_raw($id)),
				title: Set(::std::string::String::new()),
				tagline: Set(::std::string::String::new()),
				overview: Set(::std::string::String::new()),
				date: Set(::chrono::NaiveDate::from_yo_opt(1, 1).expect("from_yo_opt")),
				sort_title: Set(::std::string::String::new()),
				fs_slug: Set(format!("S FS {}", $id)),
				web_slug: Set(format!("S Web {}", $id)),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_info_show;

	macro_rules! make_info_season {
		($db:expr, $show:expr, $season:expr) => {
			$crate::entity::info::seasons::ActiveModel {
				show_id: Set(::flix_model::id::ShowId::from_raw($show)),
				season_number: Set(::flix_model::numbers::SeasonNumber::new($season)),
				title: Set(::std::string::String::new()),
				overview: Set(::std::string::String::new()),
				date: Set(::chrono::NaiveDate::from_yo_opt(1, 1).expect("from_yo_opt")),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_info_season;

	macro_rules! make_info_episode {
		($db:expr, $show:expr, $season:expr, $episode:expr) => {
			$crate::entity::info::episodes::ActiveModel {
				show_id: Set(::flix_model::id::ShowId::from_raw($show)),
				season_number: Set(::flix_model::numbers::SeasonNumber::new($season)),
				episode_number: Set(::flix_model::numbers::EpisodeNumber::new($episode)),
				title: Set(::std::string::String::new()),
				overview: Set(::std::string::String::new()),
				date: Set(::chrono::NaiveDate::from_yo_opt(1, 1).expect("from_yo_opt")),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_info_episode;
}

#[cfg(test)]
mod tests {
	use flix_model::id::{CollectionId, MovieId, ShowId};

	use chrono::NaiveDate;
	use sea_orm::ActiveValue::{NotSet, Set};
	use sea_orm::entity::prelude::*;
	use sea_orm::sqlx::error::ErrorKind;

	use crate::tests::new_initialized_memory_db;

	use super::super::tests::get_error_kind;
	use super::super::tests::notsettable;
	use super::test::{
		make_info_collection, make_info_episode, make_info_movie, make_info_season, make_info_show,
	};

	#[tokio::test]
	async fn use_test_macros() {
		let db = new_initialized_memory_db().await;

		make_info_collection!(&db, 1);
		make_info_movie!(&db, 1);
		make_info_show!(&db, 1);
		make_info_season!(&db, 1, 1);
		make_info_episode!(&db, 1, 1, 1);
	}

	#[tokio::test]
	async fn test_round_trip_collections() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_collection {
			($db:expr, $id:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_collection!(@insert, $db, $id $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, CollectionId::from_raw($id));
				assert_eq!(model.title, concat!("C Title ", $id));
				assert_eq!(model.overview, concat!("C Overview ", $id));
			};
			($db:expr, $id:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_collection!(@insert, $db, $id $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal $(; $($skip:ident),+)?) => {
				super::collections::ActiveModel {
					id: notsettable!(id, CollectionId::from_raw($id) $(, $($skip),+)?),
					title: notsettable!(title, concat!("C Title ", $id).to_string() $(, $($skip),+)?),
					overview: notsettable!(overview, concat!("C Overview ", $id).to_string() $(, $($skip),+)?),
					sort_title: notsettable!(sort_title, concat!("C Sort Title ", $id).to_string() $(, $($skip),+)?),
					fs_slug: notsettable!(fs_slug, concat!("C FS Slug ", $id).to_string() $(, $($skip),+)?),
					web_slug: notsettable!(web_slug, concat!("C Web Slug ", $id).to_string() $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		assert_collection!(&db, 1, Success);
		assert_collection!(&db, 1, UniqueViolation);
		assert_collection!(&db, 2, Success);

		assert_collection!(&db, 3, Success; id);
		assert_collection!(&db, 4, NotNullViolation; title);
		assert_collection!(&db, 5, NotNullViolation; overview);
		assert_collection!(&db, 6, NotNullViolation; sort_title);
		assert_collection!(&db, 7, NotNullViolation; fs_slug);
		assert_collection!(&db, 8, NotNullViolation; web_slug);
	}

	#[tokio::test]
	async fn test_round_trip_movies() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_movie {
			($db:expr, $id:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_movie!(@insert, $db, $id $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, MovieId::from_raw($id));
				assert_eq!(model.title, concat!("M Title ", $id));
				assert_eq!(model.tagline, concat!("M Tagline ", $id));
				assert_eq!(model.overview, concat!("M Overview ", $id));
				assert_eq!(model.date, NaiveDate::from_yo_opt($id, 1).expect("from_yo_opt"));
			};
			($db:expr, $id:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_movie!(@insert, $db, $id $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $id:literal $(; $($skip:ident),+)?) => {
				super::movies::ActiveModel {
					id: notsettable!(id, MovieId::from_raw($id) $(, $($skip),+)?),
					title: notsettable!(title, concat!("M Title ", $id).to_string() $(, $($skip),+)?),
					tagline: notsettable!(tagline, concat!("M Tagline ", $id).to_string() $(, $($skip),+)?),
					overview: notsettable!(overview, concat!("M Overview ", $id).to_string() $(, $($skip),+)?),
					date: notsettable!(date, NaiveDate::from_yo_opt($id, 1).expect("from_yo_opt") $(, $($skip),+)?),
					sort_title: notsettable!(sort_title, concat!("M Sort Title ", $id).to_string() $(, $($skip),+)?),
					fs_slug: notsettable!(fs_slug, concat!("M FS Slug ", $id).to_string() $(, $($skip),+)?),
					web_slug: notsettable!(web_slug, concat!("M Web Slug ", $id).to_string() $(, $($skip),+)?),
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
		assert_movie!(&db, 8, NotNullViolation; sort_title);
		assert_movie!(&db, 9, NotNullViolation; fs_slug);
		assert_movie!(&db, 10, NotNullViolation; web_slug);
	}

	#[tokio::test]
	async fn test_round_trip_shows() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_show {
			($db:expr, $id:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_show!(@insert, $db, $id $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, ShowId::from_raw($id));
				assert_eq!(model.title, concat!("S Title ", $id));
				assert_eq!(model.tagline, concat!("S Tagline ", $id));
				assert_eq!(model.overview, concat!("S Overview ", $id));
				assert_eq!(model.date, NaiveDate::from_yo_opt($id, 1).expect("from_yo_opt"));
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
					title: notsettable!(title, concat!("S Title ", $id).to_string() $(, $($skip),+)?),
					tagline: notsettable!(tagline, concat!("S Tagline ", $id).to_string() $(, $($skip),+)?),
					overview: notsettable!(overview, concat!("S Overview ", $id).to_string() $(, $($skip),+)?),
					date: notsettable!(date, NaiveDate::from_yo_opt($id, 1).expect("from_yo_opt") $(, $($skip),+)?),
					sort_title: notsettable!(sort_title, concat!("S Sort Title ", $id).to_string() $(, $($skip),+)?),
					fs_slug: notsettable!(fs_slug, concat!("S FS Slug ", $id).to_string() $(, $($skip),+)?),
					web_slug: notsettable!(web_slug, concat!("S Web Slug ", $id).to_string() $(, $($skip),+)?),
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
		assert_show!(&db, 8, NotNullViolation; sort_title);
		assert_show!(&db, 9, NotNullViolation; fs_slug);
		assert_show!(&db, 10, NotNullViolation; web_slug);
	}

	#[tokio::test]
	async fn test_round_trip_seasons() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_season {
			($db:expr, $show:literal, $season:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_season!(@insert, $db, $show, $season $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.show_id, ShowId::from_raw($show));
				assert_eq!(model.season_number, ::flix_model::numbers::SeasonNumber::new($season));
				assert_eq!(model.title, concat!("SS Title ", $show, ",", $season));
				assert_eq!(model.overview, concat!("SS Overview ", $show, ",", $season));
				assert_eq!(model.date, NaiveDate::from_yo_opt($show + $season, 1).expect("from_yo_opt"));
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
					show_id: notsettable!(show_id, ShowId::from_raw($show) $(, $($skip),+)?),
					season_number: notsettable!(season_number, ::flix_model::numbers::SeasonNumber::new($season) $(, $($skip),+)?),
					title: notsettable!(title, concat!("SS Title ", $show, ",", $season).to_string() $(, $($skip),+)?),
					overview: notsettable!(overview, concat!("SS Overview ", $show, ",", $season).to_string() $(, $($skip),+)?),
					date: notsettable!(date, NaiveDate::from_yo_opt($show + $season, 1).expect("from_yo_opt") $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		assert_season!(&db, 1, 1, ForeignKeyViolation);
		make_info_show!(&db, 1);
		make_info_show!(&db, 2);

		assert_season!(&db, 1, 1, Success);
		assert_season!(&db, 1, 1, UniqueViolation);
		assert_season!(&db, 2, 1, Success);
		assert_season!(&db, 1, 2, Success);

		assert_season!(&db, 1, 3, NotNullViolation; show_id);
		assert_season!(&db, 1, 4, NotNullViolation; season_number);
		assert_season!(&db, 1, 5, NotNullViolation; title);
		assert_season!(&db, 1, 6, NotNullViolation; overview);
		assert_season!(&db, 1, 7, NotNullViolation; date);
	}

	#[tokio::test]
	async fn test_round_trip_episodes() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_episode {
			($db:expr, $show:literal, $season:literal, $episode:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_episode!(@insert, $db, $show, $season, $episode $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.show_id, ShowId::from_raw($show));
				assert_eq!(model.season_number, ::flix_model::numbers::SeasonNumber::new($season));
				assert_eq!(model.episode_number, ::flix_model::numbers::EpisodeNumber::new($episode));
				assert_eq!(model.title, concat!("SSE Title ", $show, ",", $season, ",", $episode));
				assert_eq!(model.overview, concat!("SSE Overview ", $show, ",", $season, ",", $episode));
				assert_eq!(model.date, NaiveDate::from_yo_opt($show + $season, 1).expect("from_yo_opt"));
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
					show_id: notsettable!(show_id, ShowId::from_raw($show) $(, $($skip),+)?),
					season_number: notsettable!(season_number, ::flix_model::numbers::SeasonNumber::new($season) $(, $($skip),+)?),
					episode_number: notsettable!(episode_number, ::flix_model::numbers::EpisodeNumber::new($episode) $(, $($skip),+)?),
					title: notsettable!(title, concat!("SSE Title ", $show, ",", $season, ",", $episode).to_string() $(, $($skip),+)?),
					overview: notsettable!(overview, concat!("SSE Overview ", $show, ",", $season, ",", $episode).to_string() $(, $($skip),+)?),
					date: notsettable!(date, NaiveDate::from_yo_opt($show + $season, 1).expect("from_yo_opt") $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		assert_episode!(&db, 1, 1, 1, ForeignKeyViolation);
		make_info_show!(&db, 1);
		make_info_show!(&db, 2);
		assert_episode!(&db, 1, 1, 1, ForeignKeyViolation);
		make_info_season!(&db, 1, 1);
		make_info_season!(&db, 1, 2);
		make_info_season!(&db, 2, 1);

		assert_episode!(&db, 1, 1, 1, Success);
		assert_episode!(&db, 1, 1, 1, UniqueViolation);
		assert_episode!(&db, 2, 1, 1, Success);
		assert_episode!(&db, 1, 2, 1, Success);
		assert_episode!(&db, 1, 1, 2, Success);

		assert_episode!(&db, 1, 1, 3, NotNullViolation; show_id);
		assert_episode!(&db, 1, 1, 4, NotNullViolation; season_number);
		assert_episode!(&db, 1, 1, 4, NotNullViolation; episode_number);
		assert_episode!(&db, 1, 1, 5, NotNullViolation; title);
		assert_episode!(&db, 1, 1, 6, NotNullViolation; overview);
		assert_episode!(&db, 1, 1, 7, NotNullViolation; date);
	}
}
