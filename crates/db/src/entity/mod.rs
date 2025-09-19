//! Entity structs for interacting with the database

pub mod content;
pub mod info;
#[cfg(feature = "tmdb")]
pub mod tmdb;
pub mod watched;

#[cfg(test)]
mod tests {
	pub use sea_orm::DbErr;
	use sea_orm::sqlx::error::ErrorKind;

	#[derive(Debug)]
	pub enum ErrorKindError {
		NotRuntimeError,
		NotSqlxError,
		NotDatabaseError,
	}

	pub fn get_error_kind(error: DbErr) -> Result<ErrorKind, ErrorKindError> {
		let runtime_err = match error {
			DbErr::Conn(runtime_err) => runtime_err,
			DbErr::Exec(runtime_err) => runtime_err,
			DbErr::Query(runtime_err) => runtime_err,
			_ => return Err(ErrorKindError::NotRuntimeError),
		};

		let sqlx_err = match runtime_err {
			sea_orm::RuntimeErr::SqlxError(sqlx_err) => sqlx_err,
			_ => return Err(ErrorKindError::NotSqlxError),
		};

		let database_err = match sqlx_err.as_ref() {
			sea_orm::SqlxError::Database(database_err) => database_err,
			_ => return Err(ErrorKindError::NotDatabaseError),
		};

		Ok(database_err.kind())
	}

	/// Helper macro for writing tests for `ActiveModel` structs where
	/// toggling [sea_orm::ActiveValue] is needed
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
	/// toggling [sea_orm::ActiveValue] is needed
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

	/// Helper macro for creating a flix library
	macro_rules! have_library {
		($db:expr, $id:literal) => {
			$crate::entity::content::libraries::ActiveModel {
				id: Set(::flix_model::id::LibraryId::from_raw($id)),
				directory: Set(::std::path::PathBuf::new().into()),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(super) use have_library;

	/// Helper macro for creating a flix collection
	macro_rules! make_flix_collection {
		($db:expr, $id:literal) => {
			$crate::entity::info::collections::ActiveModel {
				id: Set(::flix_model::id::CollectionId::from_raw($id)),
				title: Set(::std::string::String::new()),
				overview: Set(::std::string::String::new()),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(super) use make_flix_collection;
	macro_rules! have_collection {
		($db:expr, $lid:literal, $id:literal, $pid:expr) => {
			$crate::entity::tests::make_flix_collection!($db, $id);
			$crate::entity::content::collections::ActiveModel {
				id: Set(::flix_model::id::CollectionId::from_raw($id)),
				parent: Set($pid.map(::flix_model::id::CollectionId::from_raw)),
				slug: Set(::std::string::String::new()),
				library: Set(::flix_model::id::LibraryId::from_raw($lid)),
				directory: Set(::std::path::PathBuf::new().into()),
				relative_poster_path: Set(None),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(super) use have_collection;

	/// Helper macro for creating a flix movie
	macro_rules! make_flix_movie {
		($db:expr, $id:literal) => {
			$crate::entity::info::movies::ActiveModel {
				id: Set(::flix_model::id::MovieId::from_raw($id)),
				title: Set(::std::string::String::new()),
				tagline: Set(::std::string::String::new()),
				overview: Set(::std::string::String::new()),
				date: Set(
					::chrono::NaiveDate::from_yo_opt(2000, $id).expect("NaiveDate::from_yo_opt")
				),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(super) use make_flix_movie;
	macro_rules! have_movie {
		($db:expr, $lid:literal, $id:literal, $pid:expr) => {
			$crate::entity::tests::make_flix_movie!($db, $id);
			$crate::entity::content::movies::ActiveModel {
				id: Set(::flix_model::id::MovieId::from_raw($id)),
				parent: Set($pid.map(::flix_model::id::CollectionId::from_raw)),
				slug: Set(::std::string::String::new()),
				library: Set(::flix_model::id::LibraryId::from_raw($lid)),
				directory: Set(::std::path::PathBuf::new().into()),
				relative_media_path: Set(::std::path::PathBuf::new().into()),
				relative_poster_path: Set(None),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(super) use have_movie;

	/// Helper macro for creating a flix show
	macro_rules! make_flix_show {
		($db:expr, $id:literal) => {
			$crate::entity::info::shows::ActiveModel {
				id: Set(::flix_model::id::ShowId::from_raw($id)),
				title: Set(::std::string::String::new()),
				tagline: Set(::std::string::String::new()),
				overview: Set(::std::string::String::new()),
				date: Set(
					::chrono::NaiveDate::from_yo_opt(2000, $id).expect("NaiveDate::from_yo_opt")
				),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(super) use make_flix_show;
	macro_rules! have_show {
		($db:expr, $lid:literal, $id:literal, $pid:expr) => {
			$crate::entity::tests::make_flix_show!($db, $id);
			$crate::entity::content::shows::ActiveModel {
				id: Set(::flix_model::id::ShowId::from_raw($id)),
				parent: Set($pid.map(::flix_model::id::CollectionId::from_raw)),
				slug: Set(::std::string::String::new()),
				library: Set(::flix_model::id::LibraryId::from_raw($lid)),
				directory: Set(::std::path::PathBuf::new().into()),
				relative_poster_path: Set(None),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(super) use have_show;

	/// Helper macro for creating a flix season
	macro_rules! make_flix_season {
		($db:expr, $show:literal, $season:literal) => {
			$crate::entity::info::seasons::ActiveModel {
				show: Set(::flix_model::id::ShowId::from_raw($show)),
				season: Set($season),
				title: Set(::std::string::String::new()),
				overview: Set(::std::string::String::new()),
				date: Set(::chrono::NaiveDate::from_yo_opt(2000, $show + $season)
					.expect("NaiveDate::from_yo_opt")),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(super) use make_flix_season;
	macro_rules! have_season {
		($db:expr, $lid:literal, $show:literal, $season:literal) => {
			$crate::entity::tests::make_flix_season!($db, $show, $season);
			$crate::entity::content::seasons::ActiveModel {
				show: Set(::flix_model::id::ShowId::from_raw($show)),
				season: Set($season),
				slug: Set(::std::string::String::new()),
				library: Set(::flix_model::id::LibraryId::from_raw($lid)),
				directory: Set(::std::path::PathBuf::new().into()),
				relative_poster_path: Set(None),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(super) use have_season;

	/// Helper macro for creating a flix episode
	macro_rules! make_flix_episode {
		($db:expr, $show:literal, $season:literal, $episode:literal) => {
			$crate::entity::info::episodes::ActiveModel {
				show: Set(::flix_model::id::ShowId::from_raw($show)),
				season: Set($season),
				episode: Set($episode),
				title: Set(::std::string::String::new()),
				overview: Set(::std::string::String::new()),
				date: Set(::chrono::NaiveDate::from_yo_opt(2000, $show + $season)
					.expect("NaiveDate::from_yo_opt")),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(super) use make_flix_episode;
	macro_rules! have_episode {
		($db:expr, $lid:literal, $show:literal, $season:literal, $episode:literal) => {
			$crate::entity::tests::make_flix_episode!($db, $show, $season, $episode);
			$crate::entity::content::episodes::ActiveModel {
				show: Set(::flix_model::id::ShowId::from_raw($show)),
				season: Set($season),
				episode: Set($episode),
				slug: Set(::std::string::String::new()),
				library: Set(::flix_model::id::LibraryId::from_raw($lid)),
				directory: Set(::std::path::PathBuf::new().into()),
				relative_media_path: Set(::std::path::PathBuf::new().into()),
				relative_poster_path: Set(None),
			}
			.insert($db)
			.await
			.expect("insert");
		};
	}
	pub(super) use have_episode;
}
