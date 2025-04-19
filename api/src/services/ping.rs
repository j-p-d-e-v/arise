use actix_web::{HttpResponse, Responder};

pub async fn pong() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
