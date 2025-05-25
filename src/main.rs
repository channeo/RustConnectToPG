use actix_wed :: {web::data, App,Httpserver};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};



mod services;


pub struct AppState {
    pub db_pool: Pool<Postgres>,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create database pool");

    let app_state = data(AppState { db_pool });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(services::config)
    })
    .bind("127.0.0.1", 8080).
    run()
    .await
    .expect("Failed to start server");
    Ok(())
}