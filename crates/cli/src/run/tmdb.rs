use flix::model::{
	Collection, Episode, GenericCollection, GenericEpisode, GenericMovie, GenericSeason,
	GenericShow, Movie, Season, Show, TmdbCollection, TmdbEpisode, TmdbMovie, TmdbSeason, TmdbShow,
};
use flix_tmdb::Client;
use flix_tmdb::model::{
	Collection as TCollection, Episode as TEpisode, Movie as TMovie, Season as TSeason,
	Show as TShow, ShowId,
};

use anyhow::{Context, Result};

use crate::cli::tmdb::Command;

pub enum TmdbObject {
	Collection(TCollection),
	Movie(TMovie),
	Show(TShow),
	Season(TSeason, ShowId),
	Episode(TEpisode, u32, ShowId),
}

impl TmdbObject {
	pub fn serialize(self) -> Result<String> {
		Ok(match self {
			TmdbObject::Collection(tmdb) => toml::to_string(&Collection {
				collection: GenericCollection {
					title: tmdb.title,
					overview: tmdb.overview,
				},
				tmdb: Some(TmdbCollection { id: tmdb.id }),
			})?,
			TmdbObject::Movie(tmdb) => toml::to_string(&Movie {
				movie: GenericMovie {
					title: tmdb.title,
					overview: tmdb.overview,
					genres: tmdb.genres.iter().cloned().map(|g| g.name).collect(),
					release_date: tmdb.release_date,
				},
				tmdb: Some(TmdbMovie {
					id: tmdb.id,
					genres: tmdb.genres.iter().map(|g| g.id).collect(),
				}),
			})?,
			TmdbObject::Show(tmdb) => toml::to_string(&Show {
				show: GenericShow {
					title: tmdb.title,
					overview: tmdb.overview,
					genres: tmdb.genres.iter().cloned().map(|g| g.name).collect(),
					air_date: tmdb.first_air_date,
				},
				tmdb: Some(TmdbShow {
					id: tmdb.id,
					genres: tmdb.genres.iter().map(|g| g.id).collect(),
				}),
			})?,
			TmdbObject::Season(tmdb, show_id) => toml::to_string(&Season {
				season: GenericSeason {
					number: tmdb.season_number,
					title: tmdb.title,
					overview: tmdb.overview,
					air_date: tmdb.air_date,
				},
				tmdb: Some(TmdbSeason { show_id }),
			})?,
			TmdbObject::Episode(tmdb, season_number, show_id) => toml::to_string(&Episode {
				episode: GenericEpisode {
					number: tmdb.episode_number,
					season: season_number,
					title: tmdb.title,
					overview: tmdb.overview,
					air_date: tmdb.air_date,
				},
				tmdb: Some(TmdbEpisode { show_id }),
			})?,
		})
	}
}

impl TmdbObject {
	pub async fn fetch(client: &Client, command: &Command) -> Result<Self> {
		Ok(match *command {
			Command::Collection { id } => Self::Collection(
				client
					.collections()
					.get_details(id, None)
					.await
					.with_context(|| format!("could not get collection details for '{id}'"))?,
			),
			Command::Movie { id } => Self::Movie(
				client
					.movies()
					.get_details(id, None)
					.await
					.with_context(|| format!("could not get movie details for '{id}'"))?,
			),
			Command::Show { id } => Self::Show(
				client
					.shows()
					.get_details(id, None)
					.await
					.with_context(|| format!("could not get show details for '{id}'"))?,
			),
			Command::Season { id, season } => Self::Season(
				client
					.seasons()
					.get_details(id, season, None)
					.await
					.with_context(|| format!("could not get show details for '{id}' S{season}"))?,
				id.into(),
			),
			Command::Episode {
				id,
				season,
				episode,
			} => Self::Episode(
				client
					.episodes()
					.get_details(id, season, episode, None)
					.await
					.with_context(|| {
						format!("could not get show details for '{id}' S{season}E{episode}")
					})?,
				season,
				id.into(),
			),
		})
	}
}
