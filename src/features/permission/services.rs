use serde_json::Value;
use crate::features::permission::repositories::PermissionRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct PermissionService;

impl PermissionService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        PermissionRepository::get(db, options).await
    }
}
