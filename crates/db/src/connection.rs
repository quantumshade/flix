//! Types and functions related to [DatabaseConnection]s

use sea_orm::{DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait as _;

/// A newtype wrapping a [DatabaseConnection]
pub struct Connection(DatabaseConnection);

impl Connection {
	/// Helper function for applying database migrations while wrapping a
	/// [DatabaseConnection] in a newtype
	pub async fn try_from(db: DatabaseConnection) -> Result<Self, DbErr> {
		crate::migration::Migrator::down(&db, None).await?;
		db.get_schema_registry("flix_db::*").sync(&db).await?;
		db.get_schema_registry("flix_db::*").sync(&db).await?;
		crate::migration::Migrator::up(&db, None).await?;
		Ok(Self(db))
	}
}

impl AsRef<DatabaseConnection> for Connection {
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
