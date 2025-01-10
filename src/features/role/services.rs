use serde_json::Value;
use crate::features::role::repositories::RoleRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct RoleService;

impl RoleService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        RoleRepository::get(db, options).await
    }
}
