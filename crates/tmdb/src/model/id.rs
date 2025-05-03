use core::fmt;
use core::marker::PhantomData;

/// The TMDB ID type of a movie genre
pub type MovieGenreId = TmdbId<MovieGenre>;
/// The TMDB ID type of a show genre
pub type ShowGenreId = TmdbId<ShowGenre>;
/// The TMDB ID type of a collection
pub type CollectionId = TmdbId<Collection>;
/// The TMDB ID type of a movie
pub type MovieId = TmdbId<Movie>;
/// The TMDB ID type of a show
pub type ShowId = TmdbId<Show>;

pub enum MovieGenre {}
pub enum ShowGenre {}
pub enum Collection {}
pub enum Movie {}
pub enum Show {}

type Inner = i32;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct TmdbId<T> {
	inner: Inner,
	#[serde(skip_serializing, default)]
	_phantom: PhantomData<T>,
}

impl<T> TmdbId<T> {
	pub fn inner(&self) -> Inner {
		self.inner
	}
}

impl<T> From<Inner> for TmdbId<T> {
	fn from(value: Inner) -> Self {
		Self {
			inner: value,
			_phantom: PhantomData,
		}
	}
}

impl<T> From<TmdbId<T>> for Inner {
	fn from(value: TmdbId<T>) -> Self {
		value.inner
	}
}

impl<T> fmt::Debug for TmdbId<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.inner.fmt(f)
	}
}

impl<T> fmt::Display for TmdbId<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.inner.fmt(f)
	}
}

impl<T> Clone for TmdbId<T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> Copy for TmdbId<T> {}

impl<T> PartialEq for TmdbId<T> {
	fn eq(&self, other: &Self) -> bool {
		self.inner == other.inner
	}
}

impl<T> Eq for TmdbId<T> {}
