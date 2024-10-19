use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn greet() -> impl Responder {
    format!("hello world")
}

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;


#[tokio::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 5800))?
    .run()
    .await
}
