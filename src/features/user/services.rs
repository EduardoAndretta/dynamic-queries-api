use serde_json::Value;
use crate::features::user::repositories::UserRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct UserService;

impl UserService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        UserRepository::get(db, options).await
    }
}
