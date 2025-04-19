pub mod config;
pub mod db;
pub mod models;
pub mod services;
use std::sync::Arc;

pub struct AppState {
    pub db: Arc<db::Db>,
}
