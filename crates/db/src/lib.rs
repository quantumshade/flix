//! flix-db provides types for storing persistent data about media

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod connection;
pub mod entity;
pub mod migration;

#[cfg(test)]
mod tests {
	use sea_orm::{ConnectOptions, Database, DatabaseConnection};

	pub async fn new_memory_db() -> DatabaseConnection {
		let options = ConnectOptions::new("sqlite:/tmp/db?mode=memory");
		Database::connect(options).await.expect("Database::connect")
	}
}
