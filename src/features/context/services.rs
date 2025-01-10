use serde_json::Value;
use crate::features::context::repositories::ContextRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct ContextService;

impl ContextService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        ContextRepository::get(db, options).await
    }
}
