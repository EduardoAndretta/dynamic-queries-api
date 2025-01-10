use serde_json::Value;
use crate::features::relation::repositories::RelationRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct RelationService;

impl RelationService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        RelationRepository::get(db, options).await
    }
}
