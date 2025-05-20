use flix::model::{Collection, Episode, Movie, Season, Show, Verse};

use anyhow::{Result, anyhow, bail};

use crate::cli::BackendCommand;
use crate::cli::tmdb;

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum FlixObject {
	Collection(Collection),
	Movie(Movie),
	Show(Show),
	Season(Season),
	Episode(Episode),
	// DEAD CODE: The TMDB backend does not support Verses
	#[allow(dead_code)]
	Verse(Verse),
}

impl FlixObject {
	pub fn backend_command(&self) -> Result<BackendCommand> {
		Ok(match self {
			FlixObject::Collection(collection) => {
				let Some(ref tmdb) = collection.tmdb else {
					bail!("missing tmdb data")
				};
				tmdb::Command::Collection { id: tmdb.id.into() }.into()
			}
			FlixObject::Movie(movie) => {
				let Some(ref tmdb) = movie.tmdb else {
					bail!("missing tmdb data")
				};
				tmdb::Command::Movie { id: tmdb.id.into() }.into()
			}
			FlixObject::Show(show) => {
				let Some(ref tmdb) = show.tmdb else {
					bail!("missing tmdb data")
				};
				tmdb::Command::Show { id: tmdb.id.into() }.into()
			}
			FlixObject::Season(season) => {
				let Some(ref tmdb) = season.tmdb else {
					bail!("missing tmdb data")
				};
				tmdb::Command::Season {
					id: tmdb.show_id.into(),
					season: season.season.number,
				}
				.into()
			}
			FlixObject::Episode(episode) => {
				let Some(ref tmdb) = episode.tmdb else {
					bail!("missing tmdb data")
				};
				tmdb::Command::Episode {
					id: tmdb.show_id.into(),
					season: episode.episode.season,
					episode: episode
						.episode
						.number
						.primary_episode_number()
						.ok_or_else(|| {
							anyhow!("the episode does not have a primary episode number")
						})?,
					episodes: episode.episode.number.additional_episode_numbers(),
				}
				.into()
			}
			FlixObject::Verse(_) => {
				bail!("verses are not handled")
			}
		})
	}
}
