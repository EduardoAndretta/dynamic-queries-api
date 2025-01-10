use serde_json::Value;
use crate::features::authentication_type::repositories::AuthenticationTypeRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct AuthenticationTypeService;

impl AuthenticationTypeService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        AuthenticationTypeRepository::get(db, options).await
    }
}
