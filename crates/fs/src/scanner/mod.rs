//! This module contains all of the filesystem scanner modules
//!
//! The most common scanner to use is [generic::Scanner] which will
//! automatically detect and use the appropriate scanner.

pub mod library;

pub mod generic;

pub mod collection;

pub mod movie;

pub mod episode;
pub mod season;
pub mod show;
