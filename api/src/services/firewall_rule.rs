use crate::models::firewall_rule::{FirewallRule, FirewallRuleData};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

pub async fn get_firewall_rules(app_state: web::Data<AppState>) -> impl Responder {
    let api = FirewallRule::new(app_state.db.clone());
    match api.list().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}
