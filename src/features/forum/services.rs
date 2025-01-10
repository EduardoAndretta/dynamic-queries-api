use serde_json::Value;
use crate::features::forum::repositories::ForumRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct ForumService;

impl ForumService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        ForumRepository::get(db, options).await
    }
}
