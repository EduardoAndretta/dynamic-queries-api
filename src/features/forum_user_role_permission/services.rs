use serde_json::Value;
use crate::features::forum_user_role_permission::repositories::Repository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct Service;

impl Service {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        Repository::get(db, options).await
    }
             
    pub async fn get_count(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Value, String> {
        
        Repository::get_count(db, options).await
    }
}
