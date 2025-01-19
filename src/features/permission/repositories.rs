use serde_json::Value;

use crate::features::permission::models::Model;
use crate::dto::query_params::QueryParams;
use crate::database::{Database, DatabaseType};

pub struct Repository;

impl Repository {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        match db {
            DatabaseType::Mssql(mssql_db) => {
                mssql_db.execute::<Model>(&options).await.map_err(|e| e.to_string())
            }
            DatabaseType::Sqlite(sqlite_db) => {
                sqlite_db.execute::<Model>(&options).await.map_err(|e| e.to_string())
            }
        }
    }
                         
    pub async fn get_count(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Value, String> {
        
        match db {
            DatabaseType::Mssql(mssql_db) => {
                mssql_db.execute_count::<Model>(&options).await.map_err(|e| e.to_string())
            }
            DatabaseType::Sqlite(sqlite_db) => {
                sqlite_db.execute_count::<Model>(&options).await.map_err(|e| e.to_string())
            }
        }
    }
}
