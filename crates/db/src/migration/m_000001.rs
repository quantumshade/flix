//! Adds watched views:
//!  - Collections
//!  - Shows
//!  - Seasons

use sea_orm::{DbErr, DeriveMigrationName};
use sea_orm_migration::async_trait;
use sea_orm_migration::{MigrationTrait, SchemaManager};

mod collections;
mod seasons;
mod shows;

#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		seasons::up(manager).await?;
		shows::up(manager).await?;
		collections::up(manager).await?;

		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		collections::down(manager).await?;
		shows::down(manager).await?;
		seasons::down(manager).await?;

		Ok(())
	}
}
