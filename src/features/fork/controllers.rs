use actix_web::{get, web, Responder, HttpResponse};
use crate::features::fork::services::ForkService;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

#[get("/api/fork")]
pub async fn get(
    options: web::Query<QueryParams>,
    db: web::Data<DatabaseType>,
) -> impl Responder {
    let result = ForkService::get(&*db, options.into_inner()).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            let error_message = format!("Error: {}", err);
            HttpResponse::BadRequest().body(error_message)
        }
    }
}
