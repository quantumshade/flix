//! Databse helpers.

use std::path::Path;

use flix::db::connection::Connection;

use anyhow::{Context as _, Result, bail};
use sea_orm::{ConnectOptions, Database};
use tokio::fs;

/// Connect to a database using a connection string.
///
/// # Errors
/// If the database cannot be opened.
async fn connect(string: String) -> Result<Connection> {
	Connection::try_from(
		Database::connect(ConnectOptions::new(string))
			.await
			.context("Database::connect")?,
	)
	.await
	.context("Connection::try_from")
}

/// Helper for opening an existing database at the given path.
///
/// # Errors
/// If the database cannot be opened.
pub(crate) async fn open(path: &Path) -> Result<Connection> {
	connect(format!("sqlite:{}?mode=rw", path.display())).await
}

/// Helper for creating then opening a database at the given path.
///
/// # Errors
/// If the database already exists or cannot be openend after creation.
pub(crate) async fn open_new(path: &Path) -> Result<Connection> {
	if fs::try_exists(path).await? {
		bail!("database already exists");
	}

	connect(format!("sqlite:{}?mode=rwc", path.display())).await
}
