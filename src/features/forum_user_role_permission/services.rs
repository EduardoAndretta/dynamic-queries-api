use serde_json::Value;
use crate::features::forum_user_role_permission::repositories::ForumUserRolePermissionRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct ForumUserRolePermissionService;

impl ForumUserRolePermissionService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        ForumUserRolePermissionRepository::get(db, options).await
    }
}
