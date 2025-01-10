use serde_json::Value;

use crate::features::forum_user_role_permission::models::ForumUserRolePermissionModel;
use crate::dto::query_params::QueryParams;
use crate::database::{Database, DatabaseType};

pub struct ForumUserRolePermissionRepository;

impl ForumUserRolePermissionRepository {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        match db {
            DatabaseType::Mssql(mssql_db) => {
                mssql_db.execute::<ForumUserRolePermissionModel>(&options).await.map_err(|e| e.to_string())
            }
            DatabaseType::Sqlite(sqlite_db) => {
                sqlite_db.execute::<ForumUserRolePermissionModel>(&options).await.map_err(|e| e.to_string())
            }
        }
    }
}
