use actix_web::{get, web, Responder, HttpResponse};
use crate::features::particular_user_role_permission::services::ParticularUserRolePermissionService;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

#[get("/api/particular-user-role-permission")]
pub async fn get(
    options: web::Query<QueryParams>,
    db: web::Data<DatabaseType>,
) -> impl Responder {
    let result = ParticularUserRolePermissionService::get(&*db, options.into_inner()).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            let error_message = format!("Error: {}", err);
            HttpResponse::BadRequest().body(error_message)
        }
    }
}
