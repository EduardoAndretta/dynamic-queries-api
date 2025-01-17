use actix_web::{web, App, HttpServer};
use crate::database::DatabaseType;

pub fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service( crate::features::action::controllers::get);
    cfg.service( crate::features::authentication_type::controllers::get);
    cfg.service( crate::features::context::controllers::get);
    cfg.service( crate::features::fork::controllers::get);
    cfg.service( crate::features::forum::controllers::get);
    cfg.service( crate::features::forum_user_role_permission::controllers::get);
    cfg.service( crate::features::particular::controllers::get);
    cfg.service( crate::features::particular_user_role_permission::controllers::get);
    cfg.service( crate::features::permission::controllers::get);
    cfg.service( crate::features::relation::controllers::get);
    cfg.service( crate::features::role::controllers::get);
    cfg.service( crate::features::topic::controllers::get);
    cfg.service( crate::features::user_authentication_type::controllers::get);
    cfg.service( crate::features::user::controllers::get);
    cfg.service( crate::features::user_system::controllers::get);
}

pub async fn run(db: DatabaseType) -> std::io::Result<()> {
    match &db {
        DatabaseType::Mssql(_) => {
            println!("Using MSSQL database");
        }
        DatabaseType::Sqlite(_) => {
            println!("Using SQLite database");
        }
    }

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(configure_services)
    })
    .bind("127.0.0.1:8900")?
    .run()
    .await;

    println!("Running Web Server on Port 'http://127.0.0.1:8900'");

    server
}
