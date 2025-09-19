use flix::db::connection::Connection;

use anyhow::{Context, Result, bail};
use sea_orm::{ConnectOptions, Database};
use tokio::fs;

async fn connect(string: String) -> Result<Connection> {
	Connection::try_from(
		Database::connect(ConnectOptions::new(string))
			.await
			.context("Database::connect")?,
	)
	.await
	.context("Connection::try_from")
}

pub async fn open(database_path: String) -> Result<Connection> {
	connect(format!("sqlite:{database_path}?mode=rw")).await
}

pub async fn open_new(database_path: String) -> Result<Connection> {
	if fs::try_exists(&database_path).await? {
		bail!("database already exists");
	}

	connect(format!("sqlite:{database_path}?mode=rwc")).await
}
