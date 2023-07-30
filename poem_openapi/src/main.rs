use std::process;

use poem::{listener::TcpListener, EndpointExt, Result, Route, Server, web::Data};
use poem_openapi::{payload::{Json, PlainText}, Object, OpenApi, OpenApiService};
use sqlx::postgres::PgPool;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("EXPLORER_PORT")
        .expect("EXPLORER_PORT must be set")
        .parse::<u16>()
        .expect("Could not parse Port. Must be valid u16.")
        .to_string();
    
    println!("Starting the server on port {:?} ...", port);
    let server_adress = "http://localhost:".to_string() + &port;
    let listener_adress = "127.0.0.1:".to_string() + &port;

    let pool = PgPool::connect(db_url.as_str()).await.expect("Failed to connect to database");
    
    println!("Connected to database");

    let api_service = OpenApiService::new(SerialnumberApi, "Serial Numbers", "0.1.0")
        .server(server_adress);
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

#[derive(Object)]
struct Serialnumberpart {
    serial: String,
    part_id: String,
    plant_id: Option<String>,
    // group: Vec<String>, //The idea came from prisma, basically select * from serialnumberpart where serialnumberpart_id = serial
    parent_id: Option<String>,
    manufacturing_date: Option<i32>, // Should be DateTime but for the life of me I couldn't figure out how. Only possible option
    shipping_date: Option<i32>,      // would be to create an intermediary Type for DateTime where the DB-DateTime is converted
    installation_date: Option<i32>,  // back and forth but that did not seem like a viable option to me. 
}

struct SerialnumberApi;

#[OpenApi]
impl SerialnumberApi {
    #[oai(path = "/serialnumberparts", method = "get")] //Why can I not get IntelliSense here?
    async fn list_serialnumber(&self, pool: Data<&PgPool>, description: PlainText<String>) -> Result<Json<Vec<Serialnumberpart>>> {

        dbg!(description);

        let serialnumberpart = sqlx::query_as!(
            Serialnumberpart,
            "select * from serialnumberpart")
            .fetch_all(pool.0)
            .await
            .unwrap();

        Ok(Json(serialnumberpart))
    }
}
