use actix_cors::Cors;
use actix_web::{get, App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin(&env::var("WEB_URL").unwrap())
            .allowed_methods(vec!["GET", "POST"]);
        App::new().wrap(cors).service(index)
    })
    .bind(env::var("API_ADDR").unwrap())?
    .run()
    .await
}
