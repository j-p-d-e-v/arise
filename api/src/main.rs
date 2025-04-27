use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use api::config::{AppConfig, DatabaServerConfig, HttpServerConfig};
use api::db::Db;
use api::services::{command_execution, firewall_rule, ping};
use api::AppState;
use clap::Parser;
use env_logger;
use std::env;
use std::sync::Arc;

#[derive(Debug, Clone, Parser)]
struct Args {
    #[arg(short, long, default_value = "Config.toml")]
    config_path: String,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();
    let config_path = args.config_path;
    let app_config = AppConfig::load(Some(config_path))?;
    let http_server_config: HttpServerConfig = app_config.http_server;
    let database_server_config: Arc<DatabaServerConfig> = Arc::new(app_config.database_server);

    unsafe {
        env::set_var("RUST_LOG", http_server_config.log_level.to_string());
    }
    env_logger::init();
    let db: Arc<Db> = Arc::new(Db::new(database_server_config).await?);
    match HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .route("/ping", web::get().to(ping::pong))
            .service(
                web::scope("/command-execution")
                    .route(
                        "/log",
                        web::post().to(command_execution::log_command_execution),
                    )
                    .route("/list", web::get().to(command_execution::executed_commands))
                    .route(
                        "/stats",
                        web::get().to(command_execution::executed_command_stats),
                    ),
            )
            .service(
                web::scope("/firewall-rule")
                    .route("/list", web::get().to(firewall_rule::get_firewall_rules)),
            )
    })
    .bind((http_server_config.host.as_str(), http_server_config.port))
    {
        Ok(server) => {
            let _ = server.workers(http_server_config.workers).run().await;
            Ok(())
        }
        Err(error) => Err(format!("[SERVER ERROR] main: {}", error.to_string())),
    }
}
