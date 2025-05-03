use std::path::Path;

use flix::model::{
	Collection, Episode, GenericCollection, GenericEpisode, GenericMovie, GenericSeason,
	GenericShow, Movie, Season, Show, TmdbCollection, TmdbMovie, TmdbShow,
};
use flix_tmdb::Client;
use flix_tmdb::model::{
	Collection as TCollection, Episode as TEpisode, Movie as TMovie, Season as TSeason,
	Show as TShow,
};

use anyhow::{Context, Result};

use crate::cli::tmdb::Command;

pub enum TmdbObject {
	Collection(TCollection),
	Movie(TMovie),
	Show(TShow),
	Season(TSeason),
	Episode(TEpisode),
}

impl TmdbObject {
	pub fn default_filename(&self) -> &'static Path {
		Path::new(match self {
			TmdbObject::Collection(_) => "collection.toml",
			TmdbObject::Movie(_) => "movie.toml",
			TmdbObject::Show(_) => "show.toml",
			TmdbObject::Season(_) => "season.toml",
			TmdbObject::Episode(_) => "episode.toml",
		})
	}

	pub fn serialize(self) -> Result<String> {
		Ok(match self {
			TmdbObject::Collection(tmdb) => toml::to_string(&Collection {
				collection: GenericCollection {
					name: tmdb.name,
					overview: tmdb.overview,
				},
				tmdb: Some(TmdbCollection { id: tmdb.id }),
			})?,
			TmdbObject::Movie(tmdb) => toml::to_string(&Movie {
				movie: GenericMovie {
					title: tmdb.title,
					overview: tmdb.overview,
					release_date: tmdb.release_date,
				},
				tmdb: Some(TmdbMovie {
					id: tmdb.id,
					collection: tmdb.collection.map(|c| c.id),
					genres: tmdb.genres.iter().map(|g| g.id).collect(),
				}),
			})?,
			TmdbObject::Show(tmdb) => toml::to_string(&Show {
				show: GenericShow {
					name: tmdb.name,
					overview: tmdb.overview,
					air_date: tmdb.first_air_date,
				},
				tmdb: Some(TmdbShow {
					id: tmdb.id,
					genres: tmdb.genres.iter().map(|g| g.id).collect(),
				}),
			})?,
			TmdbObject::Season(tmdb) => toml::to_string(&Season {
				season: GenericSeason {
					number: tmdb.season_number,
					name: tmdb.name,
					overview: tmdb.overview,
					air_date: tmdb.air_date,
				},
			})?,
			TmdbObject::Episode(tmdb) => toml::to_string(&Episode {
				episode: GenericEpisode {
					number: tmdb.episode_number,
					name: tmdb.name,
					overview: tmdb.overview,
					air_date: tmdb.air_date,
				},
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
			),
		})
	}
}
