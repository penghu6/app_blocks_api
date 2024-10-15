mod application;
mod domain;
mod infrastructure;
mod interfaces;
mod config;
mod util;
mod migrations;

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use log::{debug, info};
use crate::config::{DatabaseManager, SETTINGS};
use crate::interfaces::controller::{file_controller, home_controller, user_component_controller};
use std::sync::Arc;
use crate::config::inject_config::get_it;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    debug!("服务器地址 127.0.0.1:8080");
    debug!("database_url{}",SETTINGS.database_url);

    let db_manager = Arc::new(DatabaseManager::new());
    db_manager.configure().await;
    db_manager.run_migrations().await.expect("Failed to run database migrations");
    get_it::<DatabaseManager>().configure().await;
    HttpServer::new(move || {
        let db_manager = db_manager.clone();
        App::new()
            .app_data(web::Data::new(db_manager))
            .service(home_controller::home)
            .service(file_controller::upload_image)
            .service(user_component_controller::create)
            .service(fs::Files::new("/uploads", &SETTINGS.upload_dir).show_files_listing())
    })
    .bind(format!("127.0.0.1:{}", SETTINGS.app_port))?
    .run()
    .await
}
