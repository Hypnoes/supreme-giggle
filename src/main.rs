use actix_web::{App, HttpServer, Responder, web};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

async fn index() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Load the database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(database_url.as_str())
        .await
        .expect("Failed to connect to the database");

    println!("Connected to the database!");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the database pool
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
