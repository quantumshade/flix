//! Types and functions related to [DatabaseConnection]s

use sea_orm::{DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait as _;

/// A newtype wrapping a [DatabaseConnection]
pub struct Connection(DatabaseConnection);

impl Connection {
	/// Helper function for applying database migrations while wrapping a
	/// [DatabaseConnection] in a newtype
	pub async fn try_from(database: DatabaseConnection) -> Result<Self, DbErr> {
		crate::migration::Migrator::up(&database, None).await?;
		Ok(Self(database))
	}
}

impl AsRef<DatabaseConnection> for Connection {
	fn as_ref(&self) -> &DatabaseConnection {
		&self.0
	}
}
