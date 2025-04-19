use crate::models::command_execution::{CommandExecution, CommandExecutionData};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CommandDataForm {
    pub command: String,
    pub args: String,
    pub tgid: u32,
    pub pid: u32,
    pub gid: u32,
    pub uid: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExecutedCommandsRequest {
    pub offset: usize,
    pub limit: usize,
}
pub async fn executed_commands(
    query: web::Query<ExecutedCommandsRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let api = CommandExecution::new(app_state.db.clone());
    let q = query.into_inner();
    match api.get_executed_commands(q.limit, q.offset).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}

pub async fn executed_command_stats(app_state: web::Data<AppState>) -> impl Responder {
    let api = CommandExecution::new(app_state.db.clone());
    match api.get_counts().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}
pub async fn log_command_execution(
    json_data: web::Json<CommandDataForm>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let api = CommandExecution::new(app_state.db.clone());
    let form_data = json_data.into_inner();
    match api
        .create(CommandExecutionData {
            command: form_data.command,
            args: form_data.args,
            tgid: form_data.tgid,
            gid: form_data.gid,
            pid: form_data.pid,
            uid: form_data.uid,
            ..Default::default()
        })
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}
