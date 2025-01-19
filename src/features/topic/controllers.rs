use actix_web::{get, web, Responder, HttpResponse};
use crate::features::topic::services::Service;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

#[get("/api/topic")]
pub async fn get(
    options: web::Query<QueryParams>,
    db: web::Data<DatabaseType>,
) -> impl Responder {
    let result = Service::get(&*db, options.into_inner()).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            let error_message = format!("Error: {}", err);
            HttpResponse::BadRequest().body(error_message)
        }
    }
}

#[get("/api/topic/$count")]
pub async fn get_count(
    options: web::Query<QueryParams>,
    db: web::Data<DatabaseType>,
) -> impl Responder {
    let result = Service::get_count(&*db, options.into_inner()).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            let error_message = format!("Error: {}", err);
            HttpResponse::BadRequest().body(error_message)
        }
    }
}
