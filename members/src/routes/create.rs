use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::member;

pub async fn create(member: web::Json<member::Member>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO members (id, firstname, surname, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        member.firstname,
        member.surname,
        Utc::now().naive_utc()
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}