use actix_web::{web, HttpResponse, Responder};
use crate::domain::member;

/// extract `Info` using serde
pub async fn create(_member: web::Json<member::Member>) -> impl Responder {
    HttpResponse::Ok().finish()
}