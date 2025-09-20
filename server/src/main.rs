use actix_web::{App, HttpServer, Responder, get, web};

#[get("/puzzles/{page}")]
async fn greet(path: web::Path<usize>) -> impl Responder {
    let page = path.into_inner();
    format!("page {page}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
