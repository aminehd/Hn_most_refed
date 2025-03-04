use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::{info, error};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::config::ClientConfig;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn produce_kafka_message(producer: web::Data<FutureProducer>) -> impl Responder {
    let payload = "test message";
    let topic = "test-topic";
    
    match producer.send(
        FutureRecord::to(topic)
            .payload(payload)
            .key("test-key"),
        Duration::from_secs(0),
    ).await {
        Ok(_) => HttpResponse::Ok().body("Message sent to Kafka"),
        Err(e) => {
            error!("Failed to send message to Kafka: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to send message to Kafka")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables and logging
    dotenv::dotenv().ok();
    env_logger::init();
    
    // Configure Kafka producer
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", std::env::var("KAFKA_BOOTSTRAP_SERVERS").unwrap_or_else(|_| "kafka:9092".to_string()))
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    
    // Configure Database connection
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres");
    
    // Run test query to verify DB connection
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await
        .expect("Failed to execute query");
    
    info!("Connected to database successfully");
    info!("Starting server at http://0.0.0.0:8080");
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(producer.clone()))
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(hello))
            .route("/health", web::get().to(health))
            .route("/kafka", web::post().to(produce_kafka_message))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}