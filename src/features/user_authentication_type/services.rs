use serde_json::Value;
use crate::features::user_authentication_type::repositories::UserAuthenticationTypeRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct UserAuthenticationTypeService;

impl UserAuthenticationTypeService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        UserAuthenticationTypeRepository::get(db, options).await
    }
}
