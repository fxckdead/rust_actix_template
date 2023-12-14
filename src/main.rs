use std::env;
use std::sync::Arc;

use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error, NoTls};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    email: String,
}

async fn get_users(client: web::Data<Arc<Client>>) -> impl Responder {
    let rows = client
        .query("SELECT id, email FROM users", &[])
        .await
        .expect("Failed to query users");

    let mut users = Vec::new();

    for row in rows {
        let id: Uuid = row.get(0); // Assuming the UUID is stored in the standard format
        let user = User {
            id,
            email: row.get(1),
        };

        users.push(user);
    }

    web::Json(users)
}

async fn greet() -> impl Responder {
    "Hello, world!"
}

async fn setup_pg_connection() -> Result<Client, Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now you can use client to query the database
    Ok(client)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info"); // Set default log level if not set
    env_logger::init();

    let db_client = setup_pg_connection()
        .await
        .expect("Failed to connect to PostgreSQL");
    let db_client = Arc::new(db_client); // Wrap the client in an Arc

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db_client.clone()))
            .route("/", web::get().to(greet))
            .route("/api/users", web::get().to(get_users))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
