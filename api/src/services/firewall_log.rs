use crate::enums::ip_protocol::IpProtocol;
use crate::models::firewall_log::{FirewallLog, FirewallLogData};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct GetFirewallLogsFilter {
    pub limit: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FirewallLogForm {
    pub ip: [u8; 4],
    pub protocol: IpProtocol,
    pub server_ip: String,
    pub dest_port: Option<u16>,
    pub source_port: Option<u16>,
    pub status: bool,
}
pub async fn get_firewall_logs(
    query: web::Query<GetFirewallLogsFilter>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let filter = query.into_inner();
    let api = FirewallLog::new(app_state.db.clone());
    match api.list(filter.limit).await {
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
        dest_port: form.dest_port,
        source_port: form.source_port,
        server_ip: form.server_ip,
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
