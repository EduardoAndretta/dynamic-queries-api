use actix_web::{get, web, Responder, HttpResponse};
use crate::features::action::services::ActionService;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

#[get("/api/action")]
pub async fn get(
    options: web::Query<QueryParams>,
    db: web::Data<DatabaseType>,
) -> impl Responder {
    let result = ActionService::get(&*db, options.into_inner()).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            let error_message = format!("Error: {}", err);
            HttpResponse::BadRequest().body(error_message)
        }
    }
}
