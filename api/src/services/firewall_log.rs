use crate::enums::ip_protocol::IpProtocol;
use crate::models::firewall_log::{FirewallLog, FirewallLogData};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct GetFirewallLogsFilter {
    pub status: bool,
    pub limit: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FirewallLogForm {
    pub ip: [u8; 4],
    pub protocol: IpProtocol,
    pub port: Option<u16>,
    pub status: bool,
}
pub async fn get_firewall_logs(
    query: web::Query<GetFirewallLogsFilter>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let filter = query.into_inner();
    let api = FirewallLog::new(app_state.db.clone());
    match api.list(filter.status, filter.limit).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}
pub async fn create_firewall_log(
    form: web::Json<FirewallLogForm>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let form = form.into_inner();
    let data = FirewallLogData {
        ip: form.ip,
        port: form.port,
        protocol: form.protocol,
        status: form.status,
        ..Default::default()
    };
    let api = FirewallLog::new(app_state.db.clone());
    match api.create(data).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}
