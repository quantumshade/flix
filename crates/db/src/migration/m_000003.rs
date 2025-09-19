//! Adds entity/content tables:
//!  - Libraries
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
mod libraries;
mod movies;
mod seasons;
mod shows;

#[allow(unused_imports)]
pub use collections::FlixCollections;
#[allow(unused_imports)]
pub use episodes::FlixEpisodes;
#[allow(unused_imports)]
pub use libraries::FlixLibraries;
#[allow(unused_imports)]
pub use movies::FlixMovies;
#[allow(unused_imports)]
pub use seasons::FlixSeasons;
#[allow(unused_imports)]
pub use shows::FlixShows;

#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		libraries::up(manager).await?;
		collections::up(manager).await?;
		movies::up(manager).await?;
		shows::up(manager).await?;
		seasons::up(manager).await?;
		episodes::up(manager).await?;

		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		episodes::down(manager).await?;
		seasons::down(manager).await?;
		shows::down(manager).await?;
		movies::down(manager).await?;
		collections::down(manager).await?;
		libraries::down(manager).await?;

		Ok(())
	}
}
