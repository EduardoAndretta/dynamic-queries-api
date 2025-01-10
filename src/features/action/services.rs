use serde_json::Value;
use crate::features::action::repositories::ActionRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct ActionService;

impl ActionService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        ActionRepository::get(db, options).await
    }
}
