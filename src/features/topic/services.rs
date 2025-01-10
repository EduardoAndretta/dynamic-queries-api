use serde_json::Value;
use crate::features::topic::repositories::TopicRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct TopicService;

impl TopicService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        TopicRepository::get(db, options).await
    }
}
