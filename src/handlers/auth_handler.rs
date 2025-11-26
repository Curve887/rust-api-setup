use actix_web::{web, HttpResponse};
use bcrypt::{hash, verify};
use sqlx::PgPool;
use uuid::Uuid;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

use crate::models::user::{RegisterRequest, LoginRequest, User};

#[derive(Serialize)]
struct LoginResponse {
    email: String,
    token: String,
}

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
    .bind(Uuid::new_v4())
    .bind(&req.email)
    .bind(&hashed)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> HttpResponse {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&req.email)
    .fetch_one(pool.get_ref())
    .await;

    if let Ok(u) = user {
        let valid = verify(&req.password, &u.password).unwrap();

        if valid {
            let claims = serde_json::json!({
                "sub": u.id,
                "email": u.email,
            });

            let secret = std::env::var("JWT_SECRET").unwrap();

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_ref()),
            )
            .unwrap();

            return HttpResponse::Ok().json(LoginResponse { email: u.email, token });
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}
