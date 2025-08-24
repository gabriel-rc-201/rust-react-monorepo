
use axum:: {
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{ PgPool};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use uuid::Uuid;

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

pub async fn get_todos(
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
pub struct TodoInput { // objeto que representa a entrada para criação de uma tarefa
    titulo: String,
    descricao: String,
    praso: NaiveDate,
}

pub async fn create_todo(
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
pub struct TodoUpdateInput { // objeto que representa a entrada para criação de uma tarefa
    titulo: Option<String>,
    descricao: Option<String>,
    praso: Option<NaiveDate>,
}

pub async fn update_todo(
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

pub async fn delete_todo(
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
