//! flix-fs provides filesystem scanners for flix media.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod macros;

mod error;

pub use error::Error;

mod item;
pub use item::Item;

pub mod scanner;
