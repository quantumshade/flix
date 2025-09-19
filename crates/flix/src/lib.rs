//! flix provides mechanisms for interacting with flix media

#![cfg_attr(docsrs, feature(doc_cfg))]

pub use flix_db as db;
pub use flix_model as model;

#[cfg(feature = "fs")]
#[cfg_attr(docsrs, doc(cfg(feature = "fs")))]
pub use flix_fs as fs;
#[cfg(feature = "tmdb")]
#[cfg_attr(docsrs, doc(cfg(feature = "tmdb")))]
pub use flix_tmdb as tmdb;
