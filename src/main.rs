use actix_web::{web::{self, Data}, App, HttpServer};

use dotenv::dotenv;
use services::{fetch_users, fetch_user_articles, create_user_article};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
mod services;
mod instruction;
pub struct AppState {
    pub db: Pool<Postgres>,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create database pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState{db:pool.clone()}))
            .service(fetch_users)
            .service(fetch_user_articles)
            .service(create_user_article)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("Failed to start server");
    Ok(())
}