use serde_json::Value;
use crate::features::particular_user_role_permission::repositories::ParticularUserRolePermissionRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct ParticularUserRolePermissionService;

impl ParticularUserRolePermissionService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        ParticularUserRolePermissionRepository::get(db, options).await
    }
}
