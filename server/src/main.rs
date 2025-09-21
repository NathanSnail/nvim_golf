use std::str::FromStr;

use actix_web::{App, HttpServer, Responder, get, web};
use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

struct AppState {
    pool: Pool<Sqlite>,
}

type Data = web::Data<AppState>;

#[get("/puzzles/{from}")]
async fn greet(data: Data, path: web::Path<u32>) -> impl Responder {
    let from = path.into_inner();
    let rows = sqlx::query!("SELECT * FROM puzzles LIMIT 10 OFFSET ?", from)
        .fetch_all(&data.pool)
        .await
        .expect("TODO");
    dbg!(rows);
    format!("from {from}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_path = std::env::var("DATABASE_URL").unwrap();
    let connection_options = SqliteConnectOptions::from_str(&db_path)
        .expect("Database URL should be a valid URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .idle_timeout(std::time::Duration::from_secs(300))
        .acquire_timeout(std::time::Duration::from_secs(300))
        .connect_with(connection_options)
        .await
        .expect("Database should be able to connect");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Database migrations should succeed");

    let state = AppState { pool };
    let data = web::Data::new(state);

    HttpServer::new(move || App::new().app_data(data.clone()).service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
