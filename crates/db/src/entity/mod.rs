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
}
