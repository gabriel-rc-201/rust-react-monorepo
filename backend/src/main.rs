mod handlers;

use axum:: {
    routing::{get, put},
    Router,
    serve
};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // carregando variáveis de ambiente
    dotenv().expect("Unable to access .env file");
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("localhost:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found on .env file");

    // estabelecendo conexão com o banco de dados
    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Failed to bind to address");

    println!("Server is running on {}", listener.local_addr().unwrap());

    let app = Router::new()
        .route("/", get(|| async {"Hello World!"}))
        .route("/todos", 
            get(handlers::get_todos)
            .post(handlers::create_todo)
        )
        .route("/todo/{id}",
            put(handlers::update_todo)
            .delete(handlers::delete_todo)
        )
        .with_state(db_pool);

    serve(listener, app)
        .await
        .expect("Error serving application");

}
