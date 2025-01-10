use serde_json::Value;

use crate::features::fork::models::ForkModel;
use crate::dto::query_params::QueryParams;
use crate::database::{Database, DatabaseType};

pub struct ForkRepository;

impl ForkRepository {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        match db {
            DatabaseType::Mssql(mssql_db) => {
                mssql_db.execute::<ForkModel>(&options).await.map_err(|e| e.to_string())
            }
            DatabaseType::Sqlite(sqlite_db) => {
                sqlite_db.execute::<ForkModel>(&options).await.map_err(|e| e.to_string())
            }
        }
    }
}
