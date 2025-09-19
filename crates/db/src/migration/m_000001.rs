//! Adds entity/info tables:
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
pub use collections::FlixInfoCollections;
#[allow(unused_imports)]
pub use episodes::FlixInfoEpisodes;
#[allow(unused_imports)]
pub use movies::FlixInfoMovies;
#[allow(unused_imports)]
pub use seasons::FlixInfoSeasons;
#[allow(unused_imports)]
pub use shows::FlixInfoShows;

#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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

		Ok(())
	}
}
