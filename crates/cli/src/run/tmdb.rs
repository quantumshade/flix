//! The `flix` runtime backend.

use alloc::collections::BTreeMap;

use flix::db::entity;
use flix::model::id::{CollectionId, MovieId, ShowId};
use flix::model::numbers::{EpisodeNumber, SeasonNumber};
use flix::model::text;
use flix::tmdb::Client;
use flix::tmdb::model::id::{
	CollectionId as TmdbCollectionId, MovieId as TmdbMovieId, ShowId as TmdbShowId,
};

use anyhow::{Context as _, Result, bail};
use chrono::{Datelike as _, Utc};
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{
	ActiveModelTrait as _, DatabaseConnection, DbErr, EntityTrait as _, TransactionError,
	TransactionTrait as _,
};

use crate::cli::AddOverrides;
use crate::cli::tmdb::Command;

/// Execute an `add` command.
///
/// # Errors
/// Forwards any database transaction failures or TMDB API failures.
#[expect(clippy::print_stderr, reason = "we want to print in a binary")]
#[expect(clippy::print_stdout, reason = "we want to print in a binary")]
pub(crate) async fn add(
	client: Client,
	db: &DatabaseConnection,
	command: Command,
	overrides: AddOverrides,
) -> Result<()> {
	match command {
		Command::Collection { id } => {
			let id = TmdbCollectionId::from_raw(id);

			let collection = entity::tmdb::collections::Entity::find_by_id(id)
				.one(db)
				.await?;
			if collection.is_some() {
				bail!("collection already exists");
			}

			let collection = client
				.collections()
				.get_details(id, None)
				.await
				.with_context(|| format!("collections().get_details({})", id.into_raw()))?;

			let title = overrides.title.unwrap_or(collection.title);
			let overview = overrides.overview.unwrap_or(collection.overview);

			let sort_title = overrides
				.sort_title
				.unwrap_or_else(|| text::make_sortable_title(&title));
			let fs_slug = overrides
				.fs_slug
				.unwrap_or_else(|| text::make_fs_slug(&title));
			let mut web_slug = overrides
				.web_slug
				.unwrap_or_else(|| text::make_web_slug(&title));

			{
				const COLLECTION_SUFFIX_TO_REMOVE: &str = "-collection";
				if web_slug.ends_with(COLLECTION_SUFFIX_TO_REMOVE) {
					web_slug.truncate(
						web_slug
							.len()
							.saturating_sub(COLLECTION_SUFFIX_TO_REMOVE.len()),
					);
				}
			}

			let result: Result<CollectionId, TransactionError<DbErr>> = db
				.transaction(|txn| {
					let title = title.clone();
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

						_ = entity::tmdb::collections::ActiveModel {
							tmdb_id: Set(id),
							flix_id: Set(flix.id),
							last_update: Set(Utc::now()),
							movie_count: Set(collection.movies.len().try_into().unwrap_or(0)),
						}
						.insert(txn)
						.await?;

						Ok(flix.id)
					})
				})
				.await;

			match result {
				Ok(_) => {}
				Err(TransactionError::Connection(err) | TransactionError::Transaction(err)) => {
					return Err(err.into());
				}
			}
			println!("Created Collection: {title}");

			Ok(())
		}
		Command::Movie { id } => {
			let id = TmdbMovieId::from_raw(id);

			let movie = entity::tmdb::movies::Entity::find_by_id(id).one(db).await?;
			if movie.is_some() {
				bail!("movie already exists");
			}

			let movie = client
				.movies()
				.get_details(id, None)
				.await
				.with_context(|| format!("movies().get_details({})", id.into_raw()))?;

			let title = overrides.title.unwrap_or(movie.title);
			let year = movie.release_date.year();

			let sort_title = overrides
				.sort_title
				.unwrap_or_else(|| text::make_sortable_title(&title));
			let fs_slug = overrides
				.fs_slug
				.unwrap_or_else(|| text::make_fs_slug_year(&title, year));
			let web_slug = overrides
				.web_slug
				.unwrap_or_else(|| text::make_web_slug_year(&title, year));

			let result: Result<MovieId, TransactionError<DbErr>> = db
				.transaction(|txn| {
					let title = title.clone();
					Box::pin(async move {
						let flix = entity::info::movies::ActiveModel {
							id: NotSet,
							title: Set(title),
							tagline: Set(movie.tagline),
							overview: Set(movie.overview),
							date: Set(movie.release_date),
							sort_title: Set(sort_title),
							fs_slug: Set(fs_slug),
							web_slug: Set(web_slug),
						}
						.insert(txn)
						.await?;

						_ = entity::tmdb::movies::ActiveModel {
							tmdb_id: Set(id),
							flix_id: Set(flix.id),
							last_update: Set(Utc::now()),
							runtime: Set(movie.runtime.into()),
							collection_id: Set(movie.collection.map(|c| c.id)),
						}
						.insert(txn)
						.await?;

						Ok(flix.id)
					})
				})
				.await;

			match result {
				Ok(_) => {}
				Err(TransactionError::Connection(err) | TransactionError::Transaction(err)) => {
					return Err(err.into());
				}
			}
			println!("Created Movie: {title} ({year})");

			Ok(())
		}
		Command::Show { id } => {
			let id = TmdbShowId::from_raw(id);

			let show = entity::tmdb::shows::Entity::find_by_id(id).one(db).await?;
			if show.is_some() {
				bail!("show already exists");
			}

			let show = client
				.shows()
				.get_details(id, None)
				.await
				.with_context(|| format!("shows().get_details({})", id.into_raw()))?;
			let mut seasons = Vec::new();
			let mut episodes = BTreeMap::new();

			for season in 1..=show.number_of_seasons {
				let season = SeasonNumber::new(season);
				let season = match client
					.seasons()
					.get_details(id, season, None)
					.await
					.with_context(|| format!("seasons().get_details({}, {season})", id.into_raw()))
				{
					Ok(season) => season,
					Err(err) => {
						eprintln!("{err}");
						continue;
					}
				};
				if season.air_date > Utc::now().naive_utc().date() {
					eprintln!(
						"skipping season ({}, {})",
						id.into_raw(),
						season.season_number
					);
					break;
				}

				let Ok(number_of_episodes) = u32::try_from(season.episodes.len()) else {
					bail!(
						"could not convert {} to an EpisodeNumber",
						season.episodes.len()
					)
				};

				let mut season_episodes = Vec::new();
				for episode in 1..=number_of_episodes {
					let episode = EpisodeNumber::new(episode);
					let episode = match client
						.episodes()
						.get_details(id, season.season_number, episode, None)
						.await
					{
						Ok(value) => value,
						Err(err) => {
							eprintln!(
								"skipping episode ({}, {}, {episode}) - {err}",
								id.into_raw(),
								season.season_number,
							);
							break;
						}
					};
					season_episodes.push(episode);
				}

				drop(episodes.insert(season.season_number, season_episodes));
				seasons.push(season);
			}

			let title = overrides.title.unwrap_or(show.title);
			let year = show.first_air_date.year();

			let sort_title = overrides
				.sort_title
				.unwrap_or_else(|| text::make_sortable_title(&title));
			let fs_slug = overrides
				.fs_slug
				.unwrap_or_else(|| text::make_fs_slug_year(&title, year));
			let web_slug = overrides
				.web_slug
				.unwrap_or_else(|| text::make_web_slug_year(&title, year));

			let result: Result<ShowId, TransactionError<DbErr>> = db
				.transaction(|txn| {
					let title = title.clone();
					Box::pin(async move {
						let flix = entity::info::shows::ActiveModel {
							id: NotSet,
							title: Set(title),
							tagline: Set(show.tagline),
							overview: Set(show.overview),
							date: Set(show.first_air_date),
							sort_title: Set(sort_title),
							fs_slug: Set(fs_slug),
							web_slug: Set(web_slug),
						}
						.insert(txn)
						.await?;

						_ = entity::tmdb::shows::ActiveModel {
							tmdb_id: Set(id),
							flix_id: Set(flix.id),
							last_update: Set(Utc::now()),
							number_of_seasons: Set(show.number_of_seasons),
						}
						.insert(txn)
						.await?;

						for season in seasons {
							drop(
								entity::info::seasons::ActiveModel {
									show_id: Set(flix.id),
									season_number: Set(season.season_number),
									title: Set(season.title),
									overview: Set(season.overview),
									date: Set(season.air_date),
								}
								.insert(txn)
								.await?,
							);

							_ = entity::tmdb::seasons::ActiveModel {
								tmdb_show: Set(id),
								tmdb_season: Set(season.season_number),
								flix_show: Set(flix.id),
								flix_season: Set(season.season_number),
								last_update: Set(Utc::now()),
							}
							.insert(txn)
							.await?;
						}

						for (season, episodes) in episodes {
							for episode in episodes {
								drop(
									entity::info::episodes::ActiveModel {
										show_id: Set(flix.id),
										season_number: Set(season),
										episode_number: Set(episode.episode_number),
										title: Set(episode.title),
										overview: Set(episode.overview),
										date: Set(episode.air_date),
									}
									.insert(txn)
									.await?,
								);

								_ = entity::tmdb::episodes::ActiveModel {
									tmdb_show: Set(id),
									tmdb_season: Set(season),
									tmdb_episode: Set(episode.episode_number),
									flix_show: Set(flix.id),
									flix_season: Set(season),
									flix_episode: Set(episode.episode_number),
									last_update: Set(Utc::now()),
									runtime: Set(episode.runtime.into()),
								}
								.insert(txn)
								.await?;
							}
						}

						Ok(flix.id)
					})
				})
				.await;

			match result {
				Ok(_) => {}
				Err(TransactionError::Connection(err) | TransactionError::Transaction(err)) => {
					return Err(err.into());
				}
			}
			println!("Created Show: {title} ({year})");

			Ok(())
		}
		Command::Season { id, season } => {
			let id = TmdbShowId::from_raw(id);
			let season_number = season;

			let Some(show) = entity::tmdb::shows::Entity::find_by_id(id).one(db).await? else {
				bail!("show does not exists");
			};

			let season = entity::tmdb::seasons::Entity::find_by_id((id, season))
				.one(db)
				.await?;
			if season.is_some() {
				bail!("season already exists");
			}

			let season = client
				.seasons()
				.get_details(id, season_number, None)
				.await
				.with_context(|| {
					format!("seasons().get_details({}, {season_number})", id.into_raw())
				})?;
			let mut episodes = Vec::new();

			let Ok(number_of_episodes) = u32::try_from(season.episodes.len()) else {
				bail!(
					"could not convert {} to an EpisodeNumber",
					season.episodes.len()
				)
			};

			for episode in 1..=number_of_episodes {
				let episode = EpisodeNumber::new(episode);
				let episode = match client
					.episodes()
					.get_details(id, season.season_number, episode, None)
					.await
				{
					Ok(value) => value,
					Err(err) => {
						eprintln!(
							"skipping episode ({}, {}, {episode}) - {err}",
							id.into_raw(),
							season.season_number,
						);
						break;
					}
				};
				episodes.push(episode);
			}

			let result: Result<(), TransactionError<DbErr>> = db
				.transaction(|txn| {
					Box::pin(async move {
						drop(
							entity::info::seasons::ActiveModel {
								show_id: Set(show.flix_id),
								season_number: Set(season_number),
								title: Set(season.title),
								overview: Set(season.overview),
								date: Set(season.air_date),
							}
							.insert(txn)
							.await?,
						);

						_ = entity::tmdb::seasons::ActiveModel {
							tmdb_show: Set(show.tmdb_id),
							tmdb_season: Set(season_number),
							flix_show: Set(show.flix_id),
							flix_season: Set(season_number),
							last_update: Set(Utc::now()),
						}
						.insert(txn)
						.await?;

						for episode in episodes {
							drop(
								entity::info::episodes::ActiveModel {
									show_id: Set(show.flix_id),
									season_number: Set(season_number),
									episode_number: Set(episode.episode_number),
									title: Set(episode.title),
									overview: Set(episode.overview),
									date: Set(episode.air_date),
								}
								.insert(txn)
								.await?,
							);

							_ = entity::tmdb::episodes::ActiveModel {
								tmdb_show: Set(show.tmdb_id),
								tmdb_season: Set(season_number),
								tmdb_episode: Set(episode.episode_number),
								flix_show: Set(show.flix_id),
								flix_season: Set(season_number),
								flix_episode: Set(episode.episode_number),
								last_update: Set(Utc::now()),
								runtime: Set(episode.runtime.into()),
							}
							.insert(txn)
							.await?;
						}

						Ok(())
					})
				})
				.await;

			match result {
				Ok(()) => {}
				Err(TransactionError::Connection(err) | TransactionError::Transaction(err)) => {
					return Err(err.into());
				}
			}
			println!(
				"Created Season: {} S{season_number}",
				show.flix_id.into_raw(),
			);

			Ok(())
		}
		Command::Episode {
			id,
			season,
			episode,
			episodes,
		} => {
			/// Fetch and store episode information.
			///
			/// # Errors
			/// Forwards any database transaction failures or TMDB API failures.
			async fn fetch_episode(
				client: &Client,
				db: &DatabaseConnection,
				flix_id: ShowId,
				tmdb_id: TmdbShowId,
				id: TmdbShowId,
				season: SeasonNumber,
				episode: EpisodeNumber,
			) -> Result<()> {
				let episode_number = episode;

				let episode = entity::tmdb::episodes::Entity::find_by_id((id, season, episode))
					.one(db)
					.await?;
				if episode.is_some() {
					bail!("episode already exists");
				}

				let episode = client
					.episodes()
					.get_details(id, season, episode_number, None)
					.await
					.with_context(|| {
						format!("episodes().get_details({}, {season})", id.into_raw())
					})?;

				let result: Result<(), TransactionError<DbErr>> = db
					.transaction(|txn| {
						Box::pin(async move {
							drop(
								entity::info::episodes::ActiveModel {
									show_id: Set(flix_id),
									season_number: Set(season),
									episode_number: Set(episode_number),
									title: Set(episode.title),
									overview: Set(episode.overview),
									date: Set(episode.air_date),
								}
								.insert(txn)
								.await?,
							);

							_ = entity::tmdb::episodes::ActiveModel {
								tmdb_show: Set(tmdb_id),
								tmdb_season: Set(season),
								tmdb_episode: Set(episode_number),
								flix_show: Set(flix_id),
								flix_season: Set(season),
								flix_episode: Set(episode_number),
								last_update: Set(Utc::now()),
								runtime: Set(episode.runtime.into()),
							}
							.insert(txn)
							.await?;

							Ok(())
						})
					})
					.await;

				match result {
					Ok(()) => {}
					Err(TransactionError::Connection(err) | TransactionError::Transaction(err)) => {
						return Err(err.into());
					}
				}
				println!(
					"Created Episode: {} S{season}E{episode_number}",
					flix_id.into_raw(),
				);

				Ok(())
			}

			let id = TmdbShowId::from_raw(id);
			let season_number = season;

			let Some(show) = entity::tmdb::shows::Entity::find_by_id(id).one(db).await? else {
				bail!("show does not exists");
			};
			let Some(_) = entity::tmdb::seasons::Entity::find_by_id((id, season))
				.one(db)
				.await?
			else {
				bail!("season does not exists");
			};

			let flix_id = show.flix_id;
			let tmdb_id = show.tmdb_id;

			fetch_episode(&client, db, flix_id, tmdb_id, id, season_number, episode).await?;
			for episode in episodes {
				fetch_episode(&client, db, flix_id, tmdb_id, id, season_number, episode).await?;
			}

			Ok(())
		}
	}
}
