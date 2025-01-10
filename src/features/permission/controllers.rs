use actix_web::{get, web, Responder, HttpResponse};
use crate::features::permission::services::PermissionService;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

#[get("/api/permission")]
pub async fn get(
    options: web::Query<QueryParams>,
    db: web::Data<DatabaseType>,
) -> impl Responder {
    let result = PermissionService::get(&*db, options.into_inner()).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            let error_message = format!("Error: {}", err);
            HttpResponse::BadRequest().body(error_message)
        }
    }
}
