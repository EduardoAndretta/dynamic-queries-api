use serde_json::Value;
use crate::features::particular::repositories::ParticularRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct ParticularService;

impl ParticularService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        ParticularRepository::get(db, options).await
    }
}
