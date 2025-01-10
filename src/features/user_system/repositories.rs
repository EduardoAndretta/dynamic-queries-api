use serde_json::Value;

use crate::features::user_system::models::UserSystemModel;
use crate::dto::query_params::QueryParams;
use crate::database::{Database, DatabaseType};

pub struct UserSystemRepository;

impl UserSystemRepository {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        match db {
            DatabaseType::Mssql(mssql_db) => {
                mssql_db.execute::<UserSystemModel>(&options).await.map_err(|e| e.to_string())
            }
            DatabaseType::Sqlite(sqlite_db) => {
                sqlite_db.execute::<UserSystemModel>(&options).await.map_err(|e| e.to_string())
            }
        }
    }
}
