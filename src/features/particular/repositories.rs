use serde_json::Value;

use crate::features::particular::models::ParticularModel;
use crate::dto::query_params::QueryParams;
use crate::database::{Database, DatabaseType};

pub struct ParticularRepository;

impl ParticularRepository {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        match db {
            DatabaseType::Mssql(mssql_db) => {
                mssql_db.execute::<ParticularModel>(&options).await.map_err(|e| e.to_string())
            }
            DatabaseType::Sqlite(sqlite_db) => {
                sqlite_db.execute::<ParticularModel>(&options).await.map_err(|e| e.to_string())
            }
        }
    }
}
