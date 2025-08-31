mod handlers;
mod entities;
use crate::handlers::AppState;

use axum:: {
    routing::{get, put},
    Router,
    serve
};
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // carregando variáveis de ambiente
    dotenv().expect("Unable to access .env file");
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("localhost:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found on .env file");

    // estabelecendo conexão com o banco de dados
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(16);
    let db_conn = Database::connect(opt)
        .await
        .expect("Failed to connect to the database");

    let app_state = AppState::new(db_conn);

    let app = Router::new()
        .route("/", get(|| async {"Hello World!"}))
        .route("/todos", 
            get(handlers::get_tarefas)
            .post(handlers::criar_tarefa)
        )
        .route("/todo/{id}",
            put(handlers::update_todo)
            .delete(handlers::delete_todo)
        )
        .with_state(app_state);

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Failed to bind to address");

    let addr = listener.local_addr().expect("Failed to get local address");

    serve(listener, app)
        .await
        .expect("Error serving application");

    println!("Server is running on {}", addr);

}
