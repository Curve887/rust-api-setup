use actix_web::{web, HttpResponse};
use bcrypt::hash;
use sqlx::PgPool;

use create::models::user::{RegisterRequest, User};
use uuid::Uuid;

pub async fn register(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterRequest>,
) -> HttpResponse {
    
    let hashed = hash(&req.password, 4).unwrap();

    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, email, password)
        VALUES ($1, $2, $3)
        RETURNING *"
    )
    .bind(Uuid::newv4())
    .bind(&req.email)
    .bind(&hashed)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e)  => HttpResponse::BadRequest().body(e.to_string())
    }
}