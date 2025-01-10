use serde_json::Value;
use crate::features::user_system::repositories::UserSystemRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct UserSystemService;

impl UserSystemService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        UserSystemRepository::get(db, options).await
    }
}
