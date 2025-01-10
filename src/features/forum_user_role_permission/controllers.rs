use actix_web::{get, web, Responder, HttpResponse};
use crate::features::forum_user_role_permission::services::ForumUserRolePermissionService;
use crate::dto::query_params::QueryParams;
use crate::database::DatabaseType;

#[get("/api/forum-user-role-permission")]
pub async fn get(
    options: web::Query<QueryParams>,
    db: web::Data<DatabaseType>,
) -> impl Responder {
    let result = ForumUserRolePermissionService::get(&*db, options.into_inner()).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            let error_message = format!("Error: {}", err);
            HttpResponse::BadRequest().body(error_message)
        }
    }
}
