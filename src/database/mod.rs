use serde_json::Value;
use crate::dto::query_params::QueryParams;
use crate::dto::metadata::EntityMetadata;

use crate::database::{mssql::database::Database as MssqlDatabase, sqlite::database::Database as SqliteDatabase};

pub mod mssql {
    pub mod common {
        pub mod context {
            pub mod mapping {
                pub mod metadata_mapping;
            }
            pub mod contextualizer;
        }
    }
    pub mod database;
}

pub mod sqlite {
    pub mod common {
        pub mod context {
            pub mod mapping {
                pub mod metadata_mapping;
            }
            pub mod contextualizer;
        }
    }
    pub mod database;
}


pub trait Database {
    async fn execute<T: EntityMetadata>(&self, options: &QueryParams) -> Result<Value, String>;
    async fn execute_count<T: EntityMetadata>(&self, options: &QueryParams) -> Result<Value, String>;
}

pub enum DatabaseType {
    Mssql(MssqlDatabase),
    Sqlite(SqliteDatabase),
}

impl Clone for DatabaseType {
    fn clone(&self) -> Self {
        match self {
            DatabaseType::Mssql(db) => DatabaseType::Mssql(MssqlDatabase(db.0.clone())),
            DatabaseType::Sqlite(db) => DatabaseType::Sqlite(SqliteDatabase(db.0.clone()))
        }
    }
}

impl Database for DatabaseType {
    async fn  execute<T: EntityMetadata>(&self, options: &QueryParams) -> Result<Value, String> {
        match self {
            DatabaseType::Mssql(db) => db.execute::<T>(options).await,
            DatabaseType::Sqlite(db) => db.execute::<T>(options).await
        }
    }

    async fn execute_count<T: EntityMetadata>(&self, options: &QueryParams) -> Result<Value, String>  {
        match self {
            DatabaseType::Mssql(db) => db.execute_count::<T>(options).await,
            DatabaseType::Sqlite(db) => db.execute_count::<T>(options).await
        }
    }
}

