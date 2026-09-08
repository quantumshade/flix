//! Types and functions related to [`DatabaseConnection`]s.

use sea_orm::{DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait as _;

/// A newtype wrapping a [`DatabaseConnection`].
#[derive(Debug)]
pub struct Connection(DatabaseConnection);

impl Connection {
	/// Helper function for applying database migrations while wrapping a
	/// [`DatabaseConnection`] in a newtype.
	///
	/// # Errors
	/// Fails if connecting or applying migrations fail.
	#[inline]
	pub async fn try_from(db: DatabaseConnection) -> Result<Self, DbErr> {
		// The migrations only create views which store no data, so it is
		// important to down before up since modifications are impossible.
		//
		// Syncing the schema registry allows all real tables to be upgraded.
		// It is important to sync twice to ensure internal consistency so that
		// the views can be recreated on the latest schema.
		crate::migration::Migrator::down(&db, None).await?;
		db.get_schema_registry("flix_db::*").sync(&db).await?;
		db.get_schema_registry("flix_db::*").sync(&db).await?;
		crate::migration::Migrator::up(&db, None).await?;
		Ok(Self(db))
	}
}

impl AsRef<DatabaseConnection> for Connection {
	#[inline]
	fn as_ref(&self) -> &DatabaseConnection {
		&self.0
	}
}

#[cfg(test)]
impl Connection {
	pub(crate) fn take(self) -> DatabaseConnection {
		self.0
	}
}
