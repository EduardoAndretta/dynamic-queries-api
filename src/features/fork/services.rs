use serde_json::Value;
use crate::features::fork::repositories::ForkRepository;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

pub struct ForkService;

impl ForkService {
    pub async fn get(
        db: &DatabaseType,
        options: QueryParams,
    ) -> Result<Vec<Value>, String> {
        
        ForkRepository::get(db, options).await
    }
}
