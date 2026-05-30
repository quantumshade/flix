use flix::db::entity;
use flix::model::id::{CollectionId, ShowId};
use flix::model::numbers::{EpisodeNumber, SeasonNumber};
use flix::model::text;

use anyhow::Result;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, TransactionError, TransactionTrait};

use crate::cli::AddOverrides;
use crate::cli::flix::AddCommand;

pub async fn add(
	db: &DatabaseConnection,
	command: AddCommand,
	overrides: AddOverrides,
) -> Result<()> {
	match command {
		AddCommand::Collection { title, overview } => {
			let result: Result<CollectionId, TransactionError<DbErr>> = db
				.transaction(|txn| {
					let title = overrides.title.unwrap_or_else(|| title.clone());

					let sort_title = overrides
						.sort_title
						.unwrap_or_else(|| text::make_sortable_title(&title));
					let fs_slug = overrides
						.fs_slug
						.unwrap_or_else(|| text::make_fs_slug(&title));
					let web_slug = overrides
						.web_slug
						.unwrap_or_else(|| text::make_web_slug(&title));

					Box::pin(async move {
						let flix = entity::info::collections::ActiveModel {
							id: NotSet,
							title: Set(title),
							overview: Set(overview),
							sort_title: Set(sort_title),
							fs_slug: Set(fs_slug),
							web_slug: Set(web_slug),
						}
						.insert(txn)
						.await?;

						Ok(flix.id)
					})
				})
				.await;

			match result {
				Ok(_) => {}
				Err(TransactionError::Connection(err)) => Err(err)?,
				Err(TransactionError::Transaction(err)) => Err(err)?,
			};
			println!("Created Collection: {}", title);

			Ok(())
		}
		AddCommand::Episode {
			show_slug,
			season_number,
			episode_number,
			title,
			overview,
			air_date,
		} => {
			let result: Result<(ShowId, SeasonNumber, EpisodeNumber), TransactionError<DbErr>> = db
				.transaction(|txn| {
					let title = overrides.title.unwrap_or_else(|| title.clone());

					Box::pin(async move {
						let show = entity::info::shows::Entity::find_by_web_slug(&show_slug)
							.one(txn)
							.await?
							.ok_or_else(|| {
								DbErr::Custom(format!("show '{}' does not exist", show_slug))
							})?;

						let flix = entity::info::episodes::ActiveModel {
							show_id: Set(show.id),
							season_number: Set(season_number),
							episode_number: Set(episode_number),
							title: Set(title),
							overview: Set(overview),
							date: Set(air_date),
						}
						.insert(txn)
						.await?;

						Ok((flix.show_id, flix.season_number, flix.episode_number))
					})
				})
				.await;

			match result {
				Ok(_) => {}
				Err(TransactionError::Connection(err)) => Err(err)?,
				Err(TransactionError::Transaction(err)) => Err(err)?,
			};
			println!("Created Episode: {}", title);

			Ok(())
		}
	}
}
