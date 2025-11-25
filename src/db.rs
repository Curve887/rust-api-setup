use sqlx::{Pool, Postgres};

pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    Pool::<Postgres>::connect(&db_url).await
}