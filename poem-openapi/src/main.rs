use dotenv::dotenv;
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPool;
use std::process;

use crate::generated::API;

pub mod generated;
pub mod prisma_openapi;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect(
        "DATABASE_URL must be set\nExample: \"postgresql://username:password@host:port/database\"",
    );
    let port = std::env::var("EXPLORER_PORT")
        .expect("EXPLORER_PORT must be set")
        .parse::<u16>()
        .expect("Could not parse Port. Must be valid u16.")
        .to_string();

    println!("Starting the server on port {:?}...", port);
    let server_adress = "http://localhost:".to_string() + &port;
    let listener_adress = "127.0.0.1:".to_string() + &port;

    let pool = PgPool::connect(db_url.as_str())
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");

    let api_service = OpenApiService::new(API, "Serial Numbers", "0.1.0").server(server_adress);
    let explorer = api_service.openapi_explorer();
    let route = Route::new()
        .nest("/", api_service)
        .nest("/explorer", explorer)
        .data(pool);

    //TODO: Logging?
    //TODO: Concurrency?
    Server::new(TcpListener::bind(listener_adress))
        .run(route)
        .await
        .unwrap_or_else(|_| {
            println!("Failed to start server.");
            process::exit(0)
        });

    println!("Poem started.")
}
