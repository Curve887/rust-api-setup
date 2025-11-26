mod db;
mod handlers;
mod models;
pub mod routes;

use crate::db::init_db;
use crate::routes::auth_routes::auth_routes;
use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = init_db().await.unwrap();

    println!("Server running at http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(auth_routes)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
