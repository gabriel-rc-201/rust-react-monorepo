use axum:: {
    extract::{Path, State},
    handler::Handler,
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json,
    Router,
    serve
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, Utc};
use uuid::Uuid;

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
            get(get_todos)
            .post(create_todo)
        )
        .route("/todo/{id}",
            get(get_todo)
            .patch(update_todo)
            .delete(delete_todo)
        )
        .with_state(db_pool);

    serve(listener, app)
        .await
        .expect("Error serving application");

}

#[derive(Debug, serde::Serialize, sqlx::Type)]
#[sqlx(type_name = "EnumStatus", rename_all = "SCREAMING_SNAKE_CASE")]
enum EnumStatus {
    TODO,
    IN_PROGRESS,
    COMPLETED
}

#[derive(Serialize, sqlx::FromRow)]
struct TodoRow {
    id: Uuid,
    usuarioid: Uuid,
    titulo: String,
    descricao: String,
    status: EnumStatus,
    createdat: NaiveDateTime,
    updatedat: NaiveDateTime,
    praso: NaiveDate,
}

async fn get_todos(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let todos = sqlx::query_as::<_, TodoRow>( "select * from tarefas order by praso")
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string()
        )})?;
    
    Ok((
        StatusCode::OK,
        json!({
            "success": true,
            "data": todos
        }).to_string(),
    ))
}

async fn get_todo(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!()
}

async fn create_todo(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!()
}

async fn update_todo(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!()
}

async fn delete_todo(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!()
}

