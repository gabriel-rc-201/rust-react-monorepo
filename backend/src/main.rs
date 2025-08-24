use axum:: {
    extract::{Path, State},
    http::StatusCode,
    routing::{get, put},
    Json,
    Router,
    serve
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use chrono::{NaiveDate, NaiveDateTime, Utc};
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
            put(update_todo)
            .delete(delete_todo)
        )
        .with_state(db_pool);

    serve(listener, app)
        .await
        .expect("Error serving application");

}

#[derive(Debug, serde::Serialize, sqlx::Type)]
#[sqlx(type_name = "\"EnumStatus\"", rename_all = "SCREAMING_SNAKE_CASE")]
enum EnumStatus {
    TODO,
    IN_PROGRESS,
    COMPLETED
}

#[derive(Serialize, sqlx::FromRow)]
struct TodoRow { // entidade que descreve o dado no banco
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

#[derive(Deserialize)]
struct TodoInput { // objeto que representa a entrada para criação de uma tarefa
    titulo: String,
    descricao: String,
    praso: NaiveDate,
}

async fn create_todo(
    State(pg_pool): State<PgPool>,
    Json(todo): Json<TodoInput>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let usuario_id: Uuid = Uuid::parse_str("141de553-8034-4ed0-86af-28d7bfc5a9a0").unwrap(); // temporário até fazer a autenticação

    let created_todo = sqlx::query_as::<_, TodoRow>(
        r#"
        INSERT INTO tarefas (usuarioid, titulo, descricao, praso, status, createdat, updatedat)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING
            id,
            usuarioid,
            titulo,
            descricao,
            praso,
            status,
            createdat,
            updatedat
    "#
    )
    .bind(usuario_id)
    .bind(todo.titulo)
    .bind(todo.descricao)
    .bind(todo.praso)
    .bind(EnumStatus::TODO)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(&pg_pool)
    .await
    .map_err(|e| {(
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": e.to_string()}).to_string()
    )})?;

    Ok((
        StatusCode::CREATED,
        json!({"success": true, "data": created_todo}).to_string()
    ))
}

#[derive(Deserialize)]
struct TodoUpdateInput { // objeto que representa a entrada para criação de uma tarefa
    titulo: Option<String>,
    descricao: Option<String>,
    praso: Option<NaiveDate>,
}

async fn update_todo(
    State(pg_pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(todo): Json<TodoUpdateInput>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let updated_todo = sqlx::query_as::<_, TodoRow>(
        r#"
        UPDATE tarefas
        SET
            titulo = $1,
            descricao = $2,
            praso = $3,
            updatedat = $4
        WHERE id = $5
        RETURNING
            id,
            usuarioid,
            titulo,
            descricao,
            praso,
            status,
            createdat,
            updatedat
    "#
    )
    .bind(todo.titulo)
    .bind(todo.descricao)
    .bind(todo.praso)
    .bind(Utc::now())
    .bind(id)
    .fetch_optional(&pg_pool)
    .await
    .map_err(|e| {(
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": e.to_string()}).to_string()
    )})?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": updated_todo}).to_string()
    ))
}

async fn delete_todo(
    State(pg_pool): State<PgPool>,
    Path(id): Path<Uuid>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query("DELETE FROM tarefas WHERE id = $1")
        .bind(id)
        .execute(&pg_pool)
        .await
        .map_err(|e| {(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string()
        )})?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "message": "tarefa deletada com sucesso"}).to_string()
    ))
}

