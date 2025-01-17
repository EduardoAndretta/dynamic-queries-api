mod config {
    pub mod app;
}

mod services;

mod database;

mod dto {
    pub mod query_params;
    pub mod metadata;
}

mod features;

use config::app;
use sqlx::{MssqlPool, SqlitePool};

use crate::database::{DatabaseType, mssql::database::Database as MssqlDatabase, sqlite::database::Database as SqliteDatabase};

// Function to configure and return the appropriate database pool
async fn configure_db(database_type: &str) -> Result<DatabaseType, String> {
    match database_type {
        "mssql" => {
            let pool = MssqlPool::connect("mssql://username:password@localhost/db_name")
                .await
                .map_err(|e| format!("MSSQL connection error: {}", e))?;
            Ok(DatabaseType::Mssql(MssqlDatabase(pool)))
        }
        "sqlite" => {
            let pool = SqlitePool::connect("sqlite:///C://Users//user//Desktop//sqlite-tools-win-x64-3470000//RendaFixa//renda-fixa.db")
                .await
                .map_err(|e| format!("SQLite connection error: {}", e))?;
            Ok(DatabaseType::Sqlite(SqliteDatabase(pool)))
        }
        _ => Err("Unsupported database type".to_string()),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database_type = std::env::var("DATABASE_TYPE").unwrap_or_else(|_| "sqlite".to_string());  // Default to sqlite if not set

    let db = configure_db(&database_type).await.unwrap();

    app::run(db).await
}
