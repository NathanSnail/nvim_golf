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
async fn greet(data: Data, path: web::Path<usize>) -> impl Responder {
    let from = path.into_inner();
    format!("from {from}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_path = "data/test.db";
    let connection_options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .idle_timeout(std::time::Duration::from_secs(300))
        .connect_with(connection_options)
        .await
        .expect("Database must connect");

    let state = AppState { pool };
    let data = web::Data::new(state);

    HttpServer::new(move || App::new().app_data(data.clone()).service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
