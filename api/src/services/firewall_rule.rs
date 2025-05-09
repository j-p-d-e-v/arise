use crate::enums::ip_protocol::IpProtocol;
use crate::models::firewall_rule::{FirewallRule, FirewallRuleData};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FirewallRuleForm {
    pub ip: [u8; 4],
    pub protocol: IpProtocol,
    pub cidr: u16,
    pub layer: u8,
    pub from_port: Option<u16>,
    pub to_port: Option<u16>,
    pub status: bool,
}
pub async fn get_firewall_rules(
    path: web::Path<u8>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let layer = path.into_inner();
    let api = FirewallRule::new(app_state.db.clone());
    match api.list(layer).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}

pub async fn create_firewall_rule(
    form: web::Json<FirewallRuleForm>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let form = form.into_inner();
    let data = FirewallRuleData {
        ip: form.ip,
        cidr: form.cidr,
        protocol: form.protocol,
        from_port: form.from_port,
        to_port: form.to_port,
        status: form.status,
        ..Default::default()
    };
    let api = FirewallRule::new(app_state.db.clone());
    match api.create(data).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}
