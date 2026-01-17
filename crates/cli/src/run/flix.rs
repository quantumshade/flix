use flix::db::entity;
use flix::model::id::CollectionId;
use flix::model::text;

use anyhow::Result;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, TransactionError, TransactionTrait};

use crate::cli::flix::AddCommand;

pub async fn add(db: &DatabaseConnection, command: AddCommand) -> Result<()> {
	match command {
		AddCommand::Collection { title, overview } => {
			let result: Result<CollectionId, TransactionError<DbErr>> = db
				.transaction(|txn| {
					let title = title.clone();

					let sort_title = text::make_sortable_title(&title);
					let fs_slug = text::make_fs_slug(&title);
					let web_slug = text::make_web_slug(&title);

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

			let flix_id = match result {
				Ok(id) => id,
				Err(TransactionError::Connection(err)) => Err(err)?,
				Err(TransactionError::Transaction(err)) => Err(err)?,
			};
			println!("Created Collection: {} [{}]", title, flix_id.into_raw());

			Ok(())
		}
	}
}
