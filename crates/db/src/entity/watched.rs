//! This module contains entities for storing watched information

/// Collection entity
pub mod collections {
	use flix_model::id::{CollectionId, RawId};

	use chrono::{DateTime, Utc};
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a watched movie
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_watched_collections")]
	pub struct Model {
		/// The collection's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub id: CollectionId,
		/// The user's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub user_id: RawId,
		/// The date this collection was watched
		pub watched_date: DateTime<Utc>,

		/// The info for this collection
		#[sea_orm(
			belongs_to,
			relation_enum = "Info",
			from = "id",
			to = "id",
			on_update = "Cascade",
			on_delete = "Cascade"
		)]
		pub info: HasOne<entity::info::collections::Entity>,
		/// The content for this collection
		#[sea_orm(belongs_to, relation_enum = "Content", from = "id", to = "id", skip_fk)]
		pub content: HasOne<entity::content::collections::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Movie entity
pub mod movies {
	use flix_model::id::{MovieId, RawId};

	use chrono::{DateTime, Utc};
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a watched movie
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_watched_movies")]
	pub struct Model {
		/// The movie's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub id: MovieId,
		/// The user's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub user_id: RawId,
		/// The date this movie was watched
		pub watched_date: DateTime<Utc>,

		/// The info for this movie
		#[sea_orm(
			belongs_to,
			from = "id",
			to = "id",
			on_update = "Cascade",
			on_delete = "Cascade"
		)]
		pub info: HasOne<entity::info::movies::Entity>,
		/// The content for this movie
		#[sea_orm(belongs_to, relation_enum = "Content", from = "id", to = "id", skip_fk)]
		pub content: HasOne<entity::content::movies::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Show entity
pub mod shows {
	use flix_model::id::{RawId, ShowId};

	use chrono::{DateTime, Utc};
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a watched movie
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_watched_shows")]
	pub struct Model {
		/// The show's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub id: ShowId,
		/// The user's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub user_id: RawId,
		/// The date this show was watched
		pub watched_date: DateTime<Utc>,

		/// The info for this show
		#[sea_orm(
			belongs_to,
			relation_enum = "Info",
			from = "id",
			to = "id",
			on_update = "Cascade",
			on_delete = "Cascade"
		)]
		pub info: HasOne<entity::info::shows::Entity>,
		/// The content for this show
		#[sea_orm(belongs_to, relation_enum = "Content", from = "id", to = "id", skip_fk)]
		pub content: HasOne<entity::content::shows::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Season entity
pub mod seasons {
	use flix_model::id::{RawId, ShowId};
	use flix_model::numbers::SeasonNumber;

	use chrono::{DateTime, Utc};
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a watched movie
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_watched_seasons")]
	pub struct Model {
		/// The season's show's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub show_id: ShowId,
		/// The season's number
		#[sea_orm(primary_key, auto_increment = false)]
		pub season_number: SeasonNumber,
		/// The user's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub user_id: RawId,
		/// The date this season was watched
		pub watched_date: DateTime<Utc>,

		/// The info for this season
		#[sea_orm(
			belongs_to,
			relation_enum = "Info",
			from = "(show_id, season_number)",
			to = "(show_id, season_number)",
			on_update = "Cascade",
			on_delete = "Cascade"
		)]
		pub info: HasOne<entity::info::seasons::Entity>,
		/// The content for this season
		#[sea_orm(
			belongs_to,
			relation_enum = "Content",
			from = "(show_id, season_number)",
			to = "(show_id, season_number)",
			skip_fk
		)]
		pub content: HasOne<entity::content::seasons::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Episode entity
pub mod episodes {
	use flix_model::id::{RawId, ShowId};
	use flix_model::numbers::{EpisodeNumber, SeasonNumber};

	use chrono::{DateTime, Utc};
	use sea_orm::entity::prelude::*;

	use crate::entity;

	/// The database representation of a watched movie
	#[sea_orm::model]
	#[derive(Debug, Clone, DeriveEntityModel)]
	#[sea_orm(table_name = "flix_watched_episodes")]
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
		/// The user's ID
		#[sea_orm(primary_key, auto_increment = false)]
		pub user_id: RawId,
		/// The date this episode was watched
		pub watched_date: DateTime<Utc>,

		/// The info for this episode
		#[sea_orm(
			belongs_to,
			relation_enum = "Info",
			from = "(show_id, season_number, episode_number)",
			to = "(show_id, season_number, episode_number)",
			on_update = "Cascade",
			on_delete = "Cascade"
		)]
		pub info: HasOne<entity::info::episodes::Entity>,
		/// The content for this episode
		#[sea_orm(
			belongs_to,
			relation_enum = "Content",
			from = "(show_id, season_number, episode_number)",
			to = "(show_id, season_number, episode_number)",
			skip_fk
		)]
		pub content: HasOne<entity::content::episodes::Entity>,
	}

	impl ActiveModelBehavior for ActiveModel {}
}

/// Macros for creating watched entities
#[cfg(test)]
pub mod test {
	macro_rules! make_watched_movie {
		($db:expr, $id:expr, $user:expr) => {
			$crate::entity::watched::movies::ActiveModel {
				id: Set(::flix_model::id::MovieId::from_raw($id)),
				user_id: Set($user),
				watched_date: Set(::chrono::Utc::now()),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_watched_movie;

	macro_rules! make_watched_episode {
		($db:expr, $show:expr, $season:expr, $episode:expr, $user:expr) => {
			$crate::entity::watched::episodes::ActiveModel {
				show_id: Set(::flix_model::id::ShowId::from_raw($show)),
				season_number: Set(::flix_model::numbers::SeasonNumber::new($season)),
				episode_number: Set(::flix_model::numbers::EpisodeNumber::new($episode)),
				user_id: Set($user),
				watched_date: Set(::chrono::Utc::now()),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(crate) use make_watched_episode;
}

#[cfg(test)]
mod tests {
	use flix_model::id::{MovieId, ShowId};

	use chrono::NaiveDate;
	use sea_orm::ActiveValue::{NotSet, Set};
	use sea_orm::Condition;
	use sea_orm::entity::prelude::*;
	use sea_orm::sqlx::error::ErrorKind;

	use crate::entity::content::test::{
		make_content_collection, make_content_episode, make_content_library, make_content_movie,
		make_content_season, make_content_show,
	};
	use crate::entity::info::test::{
		make_info_episode, make_info_movie, make_info_season, make_info_show,
	};
	use crate::tests::new_initialized_memory_db;

	use super::super::tests::get_error_kind;
	use super::super::tests::notsettable;
	use super::test::{make_watched_episode, make_watched_movie};

	#[tokio::test]
	async fn use_test_macros() {
		let db = new_initialized_memory_db().await;

		make_info_movie!(&db, 1);
		make_info_show!(&db, 1);
		make_info_season!(&db, 1, 1);
		make_info_episode!(&db, 1, 1, 1);

		make_watched_movie!(&db, 1, 1);
		make_watched_episode!(&db, 1, 1, 1, 1);
	}

	#[tokio::test]
	async fn test_round_trip_movies() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_movie {
			($db:expr, $id:literal, $uid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_movie!(@insert, $db, $id, $uid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.id, MovieId::from_raw($id));
				assert_eq!(model.user_id, $uid);
				assert_eq!(model.watched_date, NaiveDate::from_yo_opt($uid, 1).expect("from_yo_opt").and_hms_opt(0, 0, 0).expect("and_hms_opt").and_utc());
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
					watched_date: notsettable!(watched_date, NaiveDate::from_yo_opt($uid, 1).expect("from_yo_opt").and_hms_opt(0, 0, 0).expect("and_hms_opt").and_utc() $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		assert_movie!(&db, 1, 1, ForeignKeyViolation);
		make_info_movie!(&db, 1);
		assert_movie!(&db, 1, 1, Success);
		assert_movie!(&db, 1, 2, Success);

		assert_movie!(&db, 1, 1, UniqueViolation);
		make_info_movie!(&db, 2);
		assert_movie!(&db, 2, 1, Success);
		assert_movie!(&db, 2, 2, Success);

		assert_movie!(&db, 3, 1, NotNullViolation; id);
		assert_movie!(&db, 4, 1, NotNullViolation; user_id);
		assert_movie!(&db, 5, 1, NotNullViolation; watched_date);
	}

	#[tokio::test]
	async fn test_round_trip_episodes() {
		let db = new_initialized_memory_db().await;

		macro_rules! assert_episode {
			($db:expr, $show:literal, $season:literal, $episode:literal, $uid:literal, Success $(; $($skip:ident),+)?) => {
				let model = assert_episode!(@insert, $db, $show, $season, $episode, $uid $(; $($skip),+)?)
					.expect("insert");

				assert_eq!(model.show_id, ShowId::from_raw($show));
				assert_eq!(model.season_number, ::flix_model::numbers::SeasonNumber::new($season));
				assert_eq!(model.episode_number, ::flix_model::numbers::EpisodeNumber::new($episode));
				assert_eq!(model.user_id, $uid);
				assert_eq!(model.watched_date, NaiveDate::from_yo_opt($uid, 1).expect("from_yo_opt").and_hms_opt(0, 0, 0).expect("and_hms_opt").and_utc());
			};
			($db:expr, $show:literal, $season:literal, $episode:literal, $uid:literal, $error:ident $(; $($skip:ident),+)?) => {
				let model = assert_episode!(@insert, $db, $show, $season, $episode, $uid $(; $($skip),+)?)
					.expect_err("insert");

				assert_eq!(get_error_kind(model).expect("get_error_kind"), ErrorKind::$error);
			};
			(@insert, $db:expr, $show:literal, $season:literal, $episode:literal, $uid:literal $(; $($skip:ident),+)?) => {
				super::episodes::ActiveModel {
					show_id: notsettable!(show_id, ShowId::from_raw($show) $(, $($skip),+)?),
					season_number: notsettable!(season_number, ::flix_model::numbers::SeasonNumber::new($season) $(, $($skip),+)?),
					episode_number: notsettable!(episode_number, ::flix_model::numbers::EpisodeNumber::new($episode) $(, $($skip),+)?),
					user_id: notsettable!(user_id, $uid $(, $($skip),+)?),
					watched_date: notsettable!(watched_date, NaiveDate::from_yo_opt($uid, 1).expect("from_yo_opt").and_hms_opt(0, 0, 0).expect("and_hms_opt").and_utc() $(, $($skip),+)?),
				}.insert($db).await
			};
		}

		make_info_show!(&db, 1);
		make_info_season!(&db, 1, 1);
		make_info_show!(&db, 2);
		make_info_season!(&db, 2, 1);

		assert_episode!(&db, 1, 1, 1, 1, ForeignKeyViolation);
		assert_episode!(&db, 2, 1, 1, 1, ForeignKeyViolation);
		make_info_episode!(&db, 1, 1, 1);
		make_info_episode!(&db, 2, 1, 1);
		assert_episode!(&db, 1, 1, 1, 1, Success);
		assert_episode!(&db, 1, 1, 1, 2, Success);
		assert_episode!(&db, 2, 1, 1, 1, Success);

		assert_episode!(&db, 1, 1, 1, 1, UniqueViolation);

		assert_episode!(&db, 3, 1, 1, 1, NotNullViolation; show_id);
		assert_episode!(&db, 4, 1, 1, 1, NotNullViolation; season_number);
		assert_episode!(&db, 5, 1, 1, 1, NotNullViolation; episode_number);
		assert_episode!(&db, 6, 1, 1, 1, NotNullViolation; user_id);
		assert_episode!(&db, 7, 1, 1, 1, NotNullViolation; watched_date);
	}

	#[tokio::test]
	async fn test_query_seasons() {
		let db = new_initialized_memory_db().await;

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
							.add(super::seasons::Column::ShowId.eq($show))
							.add(super::seasons::Column::SeasonNumber.eq($season))
							.add(super::seasons::Column::UserId.eq($uid)),
					)
					.one(&db)
					.await
					.expect("find.filter.one")
			};
		}

		make_content_library!(&db, 1);
		make_content_show!(&db, 1, 1, None);

		make_content_season!(&db, 1, 1, 1);
		assert_season!(&db, 1, 1, 1, Unwatched);
		make_content_episode!(&db, 1, 1, 1, 1);
		assert_season!(&db, 1, 1, 1, Unwatched);
		make_watched_episode!(&db, 1, 1, 1, 1);
		assert_season!(&db, 1, 1, 1, Watched);
		assert_season!(&db, 1, 1, 2, Unwatched);
		make_content_episode!(&db, 1, 1, 1, 2);
		assert_season!(&db, 1, 1, 1, Unwatched);

		make_content_season!(&db, 1, 1, 2);
		make_content_episode!(&db, 1, 1, 2, 1);
		make_content_episode!(&db, 1, 1, 2, 2, >1);
		make_info_episode!(&db, 1, 2, 3);
		make_content_episode!(&db, 1, 1, 2, 4);
		assert_season!(&db, 1, 2, 1, Unwatched);
		assert_season!(&db, 1, 2, 2, Unwatched);
		assert_season!(&db, 1, 2, 3, Unwatched);
		make_watched_episode!(&db, 1, 2, 1, 1);
		make_watched_episode!(&db, 1, 2, 1, 2);
		make_watched_episode!(&db, 1, 2, 2, 3);
		assert_season!(&db, 1, 2, 1, Unwatched);
		assert_season!(&db, 1, 2, 2, Unwatched);
		assert_season!(&db, 1, 2, 3, Unwatched);
		make_watched_episode!(&db, 1, 2, 2, 1);
		make_watched_episode!(&db, 1, 2, 2, 2);
		make_watched_episode!(&db, 1, 2, 1, 3);
		assert_season!(&db, 1, 2, 1, Unwatched);
		assert_season!(&db, 1, 2, 2, Unwatched);
		assert_season!(&db, 1, 2, 3, Unwatched);
		make_watched_episode!(&db, 1, 2, 4, 1);
		assert_season!(&db, 1, 2, 1, Watched);
		assert_season!(&db, 1, 2, 2, Unwatched);
		assert_season!(&db, 1, 2, 3, Unwatched);
	}

	#[tokio::test]
	async fn test_query_shows() {
		let db = new_initialized_memory_db().await;

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

		make_content_library!(&db, 1);
		make_content_show!(&db, 1, 1, None);

		assert_show!(&db, 1, 1, Unwatched);
		make_content_season!(&db, 1, 1, 1);
		assert_show!(&db, 1, 1, Unwatched);
		make_content_episode!(&db, 1, 1, 1, 1);
		assert_show!(&db, 1, 1, Unwatched);
		make_watched_episode!(&db, 1, 1, 1, 1);
		assert_show!(&db, 1, 1, Watched);
		assert_show!(&db, 1, 2, Unwatched);
		make_content_episode!(&db, 1, 1, 1, 2);
		assert_show!(&db, 1, 1, Unwatched);
		assert_show!(&db, 1, 2, Unwatched);
		make_watched_episode!(&db, 1, 1, 2, 1);

		make_content_season!(&db, 1, 1, 2);
		assert_show!(&db, 1, 1, Unwatched);
		assert_show!(&db, 1, 2, Unwatched);
		make_content_episode!(&db, 1, 1, 2, 1);
		assert_show!(&db, 1, 1, Unwatched);
		assert_show!(&db, 1, 2, Unwatched);
		make_watched_episode!(&db, 1, 2, 1, 1);
		assert_show!(&db, 1, 1, Watched);
		assert_show!(&db, 1, 2, Unwatched);
	}

	#[tokio::test]
	async fn test_query_collections() {
		let db = new_initialized_memory_db().await;

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

		make_content_library!(&db, 1);
		make_content_collection!(&db, 1, 1, None);
		assert_collection!(&db, 1, 1, Unwatched);

		make_content_movie!(&db, 1, 1, Some(1));
		assert_collection!(&db, 1, 1, Unwatched);
		make_info_movie!(&db, 9999);
		make_watched_movie!(&db, 9999, 1);
		assert_collection!(&db, 1, 1, Unwatched);
		make_watched_movie!(&db, 1, 1);
		assert_collection!(&db, 1, 1, Watched);

		make_content_collection!(&db, 1, 2, Some(1));
		assert_collection!(&db, 1, 1, Watched);
		assert_collection!(&db, 2, 1, Unwatched);
		make_content_movie!(&db, 1, 2, Some(2));
		assert_collection!(&db, 1, 1, Unwatched);
		assert_collection!(&db, 2, 1, Unwatched);
		make_watched_movie!(&db, 2, 1);
		assert_collection!(&db, 1, 1, Watched);
		assert_collection!(&db, 2, 1, Watched);

		make_content_show!(&db, 1, 1, Some(2));
		assert_collection!(&db, 1, 1, Unwatched);
		assert_collection!(&db, 2, 1, Unwatched);
		make_content_season!(&db, 1, 1, 1);
		make_content_episode!(&db, 1, 1, 1, 1);
		make_watched_episode!(&db, 1, 1, 1, 1);
		assert_collection!(&db, 1, 1, Watched);
		assert_collection!(&db, 2, 1, Watched);
	}
}
