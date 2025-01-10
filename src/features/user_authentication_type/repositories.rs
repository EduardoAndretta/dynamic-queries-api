use serde_json::Value;

use crate::features::user_authentication_type::models::UserAuthenticationTypeModel;
use crate::dto::query_params::QueryParams;
use crate::database::{Database, DatabaseType};

pub struct UserAuthenticationTypeRepository;

impl UserAuthenticationTypeRepository {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        match db {
            DatabaseType::Mssql(mssql_db) => {
                mssql_db.execute::<UserAuthenticationTypeModel>(&options).await.map_err(|e| e.to_string())
            }
            DatabaseType::Sqlite(sqlite_db) => {
                sqlite_db.execute::<UserAuthenticationTypeModel>(&options).await.map_err(|e| e.to_string())
            }
        }
    }
}
