//! Typed TMDB IDs.

#![expect(clippy::module_name_repetitions, reason = "ID types end with Id")]

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

#[cfg(feature = "sea-orm")]
use sea_orm::sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr};
#[cfg(feature = "sea-orm")]
use sea_orm::{ColIdx, ColumnType, DbErr, QueryResult, TryFromU64, TryGetError, TryGetable, Value};

/// The internal representation used by TMDB.
pub type TmdbRepr = u32;

/// An opaque type representing a TMDB ID.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Id<T> {
	/// Internal ID representation.
	id: TmdbRepr,
	/// Marker to indicate an invariant T.
	#[serde(skip_serializing, default)]
	_phantom: PhantomData<fn(T) -> T>,
}

// Manual implementation since `T: Clone` is not required
#[expect(clippy::missing_trait_methods, reason = "accept default methods")]
impl<T> Clone for Id<T> {
	#[inline]
	fn clone(&self) -> Self {
		*self
	}
}

// Manual implementation since `T: Copy` is not required
impl<T> Copy for Id<T> {}

// Manual implementation since `T: PartialEq` is not required
#[expect(clippy::missing_trait_methods, reason = "accept default methods")]
impl<T> PartialEq for Id<T> {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

// Manual implementation since `T: Eq` is not required
#[expect(clippy::missing_trait_methods, reason = "accept default methods")]
impl<T> Eq for Id<T> {}

// Manual implementation since `T: PartialOrd` is not required
#[expect(clippy::missing_trait_methods, reason = "accept default methods")]
impl<T> PartialOrd for Id<T> {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

// Manual implementation since `T: Ord` is not required
#[expect(clippy::missing_trait_methods, reason = "accept default methods")]
impl<T> Ord for Id<T> {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		self.id.cmp(&other.id)
	}
}

// Manual implementation since `T: Hash` is not required
#[expect(clippy::missing_trait_methods, reason = "accept default methods")]
impl<T> Hash for Id<T> {
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

impl<T> Id<T> {
	/// Allows the conversion from a raw value to [Id], though the use is discouraged.
	#[inline]
	#[must_use]
	pub fn from_raw(raw: TmdbRepr) -> Self {
		Self {
			id: raw,
			_phantom: PhantomData,
		}
	}

	/// Allows extracting the raw value, though the use is discouraged.
	#[inline]
	#[must_use]
	pub const fn into_raw(self) -> TmdbRepr {
		self.id
	}
}

impl<T> fmt::Debug for Id<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Id")
			.field("T", &core::any::type_name::<T>())
			.field("id", &self.id)
			.finish()
	}
}

#[cfg(feature = "sea-orm")]
#[expect(clippy::missing_trait_methods, reason = "accept default methods")]
impl<T> ValueType for Id<T> {
	#[inline]
	fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
		<TmdbRepr as ValueType>::try_from(v).map(|id| Self {
			id,
			_phantom: PhantomData,
		})
	}

	#[inline]
	fn type_name() -> String {
		format!("Id<{}>", core::any::type_name::<T>())
	}

	#[inline]
	fn array_type() -> ArrayType {
		TmdbRepr::array_type()
	}

	#[inline]
	fn column_type() -> ColumnType {
		TmdbRepr::column_type()
	}
}

#[cfg(feature = "sea-orm")]
impl<T> From<Id<T>> for Value {
	#[inline]
	fn from(value: Id<T>) -> Self {
		value.id.into()
	}
}

#[cfg(feature = "sea-orm")]
#[expect(clippy::missing_trait_methods, reason = "accept default methods")]
impl<T> TryGetable for Id<T> {
	#[inline]
	fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
		TmdbRepr::try_get_by(res, index).map(|id| Self {
			id,
			_phantom: PhantomData,
		})
	}
}

#[cfg(feature = "sea-orm")]
impl<T> TryFromU64 for Id<T> {
	#[inline]
	fn try_from_u64(n: u64) -> Result<Self, DbErr> {
		TmdbRepr::try_from_u64(n).map(|id| Self {
			id,
			_phantom: PhantomData,
		})
	}
}

#[cfg(feature = "sea-orm")]
impl<T> Nullable for Id<T> {
	#[inline]
	fn null() -> Value {
		TmdbRepr::null()
	}
}

#[cfg(test)]
mod tests {
	#[cfg(feature = "sea-orm")]
	#[expect(dead_code, reason = "structs test derive macros")]
	mod test_seaorm {
		use sea_orm::{
			ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
			EnumIter, PrimaryKeyTrait,
		};

		use super::super::Id;

		#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
		#[sea_orm(table_name = "ids")]
		#[expect(clippy::use_self, reason = "derive macros cause Self to be invalid")]
		struct Model {
			#[sea_orm(primary_key, auto_increment = false)]
			id: Id<Model>,
			nullable: Option<Id<Model>>,
		}

		#[expect(clippy::missing_trait_methods, reason = "accept default methods")]
		impl ActiveModelBehavior for ActiveModel {}

		#[derive(Debug, EnumIter, DeriveRelation)]
		#[expect(clippy::empty_enums, reason = "no relations at the moment")]
		enum Relation {}
	}

	#[test]
	fn serde() {
		use super::Id;

		let id: Id<()> = Id::from_raw(1234);
		serde_test::assert_tokens(&id, &[serde_test::Token::U32(1234)]);
	}
}

#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[expect(clippy::empty_enums, reason = "support rust <1.100")]
pub enum Collection {}
/// Type alias for a collection ID.
pub type CollectionId = Id<Collection>;

impl From<CollectionId> for flix_model::id::CollectionId {
	#[inline]
	fn from(value: CollectionId) -> Self {
		Self::from_raw(value.into_raw().into())
	}
}

impl TryFrom<flix_model::id::CollectionId> for CollectionId {
	type Error = <TmdbRepr as TryFrom<flix_model::id::RawId>>::Error;

	#[inline]
	fn try_from(value: flix_model::id::CollectionId) -> Result<Self, Self::Error> {
		value.into_raw().try_into().map(Self::from_raw)
	}
}

#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[expect(clippy::empty_enums, reason = "support rust <1.100")]
pub enum Movie {}
/// Type alias for a movie ID.
pub type MovieId = Id<Movie>;

impl From<MovieId> for flix_model::id::MovieId {
	#[inline]
	fn from(value: MovieId) -> Self {
		Self::from_raw(value.into_raw().into())
	}
}

impl TryFrom<flix_model::id::MovieId> for MovieId {
	type Error = <TmdbRepr as TryFrom<flix_model::id::RawId>>::Error;

	#[inline]
	fn try_from(value: flix_model::id::MovieId) -> Result<Self, Self::Error> {
		value.into_raw().try_into().map(Self::from_raw)
	}
}

#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[expect(clippy::empty_enums, reason = "support rust <1.100")]
pub enum Show {}
/// Type alias for a show ID.
pub type ShowId = Id<Show>;

impl From<ShowId> for flix_model::id::ShowId {
	#[inline]
	fn from(value: ShowId) -> Self {
		Self::from_raw(value.into_raw().into())
	}
}

impl TryFrom<flix_model::id::ShowId> for ShowId {
	type Error = <TmdbRepr as TryFrom<flix_model::id::RawId>>::Error;

	#[inline]
	fn try_from(value: flix_model::id::ShowId) -> Result<Self, Self::Error> {
		value.into_raw().try_into().map(Self::from_raw)
	}
}
