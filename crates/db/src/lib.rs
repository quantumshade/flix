//! flix-db provides types for storing persistent data about media

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod connection;
pub mod entity;
pub mod migration;

#[cfg(test)]
mod tests {
	use sea_orm::{ConnectOptions, Database, DatabaseConnection};

	use crate::connection::Connection;

	pub async fn new_initialized_memory_db() -> DatabaseConnection {
		let options = ConnectOptions::new("sqlite::memory:");

		let db = Database::connect(options)
			.await
			.expect("Database::connect()");
		let connection = Connection::try_from(db)
			.await
			.expect("Connection::try_from");
		connection.take()
	}
}
