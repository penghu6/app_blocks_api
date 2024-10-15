use crate::migrations::Migrator;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use log::debug;
use tokio::sync::RwLock;
use crate::config::SETTINGS;

pub struct DatabaseManager {
    connection: RwLock<Option<Arc<DatabaseConnection>>>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self {
            connection: RwLock::new(None),
        }
    }

    pub async fn configure(&self) {

        let mut options = ConnectOptions::new(SETTINGS.database_url.clone());

        options
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("public");

        let connection = Database::connect(options)
            .await
            .expect("Database connection failed");

        // 设置时区
        connection
            .execute_unprepared("SET TIME ZONE 'Asia/Shanghai';")
            .await
            .expect("Failed to set timezone");

        let mut connection_write = self.connection.write().await;
        connection_write.replace(Arc::new(connection));
    }

    pub async fn get(&self) -> Arc<DatabaseConnection> {
        let connection_read = self.connection.read().await;
        connection_read.clone().expect("Database connection not configured")
    }

    pub async fn run_migrations(&self) -> Result<(), DbErr> {
        let connection = self.get().await;
        Migrator::up(connection.as_ref(), None).await
    }
}
