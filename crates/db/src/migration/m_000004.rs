//! Adds entity/watched tables:
//!  - Collections
//!  - Movies
//!  - Shows
//!  - Seasons
//!  - Episodes

use sea_orm::{DbErr, DeriveMigrationName};
use sea_orm_migration::async_trait;
use sea_orm_migration::{MigrationTrait, SchemaManager};

mod collections;
mod episodes;
mod movies;
mod seasons;
mod shows;

#[allow(unused_imports)]
pub use episodes::FlixWatchedEpisodes;
#[allow(unused_imports)]
pub use movies::FlixWatchedMovies;

#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		episodes::up(manager).await?;
		seasons::up(manager).await?;
		shows::up(manager).await?;
		movies::up(manager).await?;
		collections::up(manager).await?;

		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		collections::down(manager).await?;
		movies::down(manager).await?;
		shows::down(manager).await?;
		seasons::down(manager).await?;
		episodes::down(manager).await?;

		Ok(())
	}
}
