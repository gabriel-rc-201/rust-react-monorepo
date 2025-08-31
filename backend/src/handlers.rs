use axum:: {
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;
use crate::entities::{ prelude::Tarefas, tarefas };
use crate::entities::sea_orm_active_enums::EnumStatus;

#[allow(non_camel_case_types)]
#[derive(Debug, serde::Serialize)]
enum EnumStatusDTO {
    TODO,
    IN_PROGRESS,
    COMPLETED
}

#[derive(Serialize)]
struct TarefaDTO { // entidade que descreve o dado no banco
    id: Uuid,
    usuarioid: Uuid,
    titulo: String,
    descricao: String,
    status: EnumStatusDTO,
    createdat: NaiveDateTime,
    updatedat: NaiveDateTime,
    praso: NaiveDate,
}

const USUARIO_ID_MOCK: &str = "0f8d6d1f-272e-43e9-ad54-2351bdec5192";

pub async fn get_tarefas(
    State(app_state): State<AppState>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let usuario_id = Uuid::parse_str(USUARIO_ID_MOCK).unwrap();

    let tarefas_db = Tarefas::find()
        .filter(tarefas::Column::UsuarioId.eq(usuario_id))
        .order_by_asc(tarefas::Column::Praso)
        .all(&app_state.db_conn)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string()
        ))?;

    let tarefas_dto: Vec<TarefaDTO> = tarefas_db.iter().map(|tarefa| {
        to_tarefa_dto(tarefa.clone())
    }).collect();

    Ok((
        StatusCode::OK,
        json!({
            "success": true,
            "data": tarefas_dto
        }).to_string(),
    ))
}

#[derive(Deserialize)]
pub struct TodoInput { // objeto que representa a entrada para criação de uma tarefa
    titulo: String,
    descricao: String,
    praso: NaiveDate,
}

pub async fn criar_tarefa(
    State(app_state): State<AppState>,
    Json(nova_tarefa): Json<TodoInput>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let usuario_id: Uuid = Uuid::parse_str(USUARIO_ID_MOCK).unwrap(); // temporário até fazer a autenticação

    let tarefa_criada = tarefas::ActiveModel {
        usuario_id: ActiveValue::set(usuario_id),
        titulo: ActiveValue::set(nova_tarefa.titulo),
        descricao: ActiveValue::set(nova_tarefa.descricao),
        praso: ActiveValue::set(nova_tarefa.praso),
        ..Default::default()
    };

    let tarefa_model = tarefa_criada.insert(&app_state.db_conn)
        .await
        .map_err(|e| {(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string()
        )})?;

    let tarefa_dto = to_tarefa_dto(tarefa_model);
    Ok((
        StatusCode::CREATED,
        json!({"success": true, "data": tarefa_dto}).to_string()
    ))
}

#[derive(Deserialize)]
pub struct TodoUpdateInput { // objeto que representa a entrada para criação de uma tarefa
    titulo: Option<String>,
    descricao: Option<String>,
    praso: Option<NaiveDate>,
}

pub async fn update_todo(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(tarefa): Json<TodoUpdateInput>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let tarefa_bd: tarefas::Model = Tarefas::find_by_id(id).one(&app_state.db_conn)
        .await
        .map_err(|e| {(
            StatusCode::NOT_FOUND,
            json!({"success": false, "message": e.to_string()}).to_string()
        )})?.expect("tarefa não encontrada");

    let mut tarefa_active_model = tarefa_bd.into_active_model();

    if let Some(titulo) = tarefa.titulo {
        tarefa_active_model.titulo = ActiveValue::set(titulo);
    }
    if let Some(descricao) = tarefa.descricao {
        tarefa_active_model.descricao = ActiveValue::set(descricao);
    }
    if let Some(praso) = tarefa.praso {
        tarefa_active_model.praso = ActiveValue::set(praso);
    }

    let tarefa_atualizada = tarefa_active_model.update(&app_state.db_conn)
        .await
        .map_err(|e| {(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string()
        )})?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": to_tarefa_dto(tarefa_atualizada)}).to_string()
    ))
}

pub async fn delete_todo(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    Tarefas::delete_by_id(id)
        .exec(&app_state.db_conn)
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

#[derive(Clone)]
pub struct AppState {
    db_conn: DatabaseConnection
}

impl AppState {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        AppState { db_conn: (db_conn) }
    }
}

fn to_enum_status_dto(status: EnumStatus) -> EnumStatusDTO {
    match status {
        EnumStatus::Todo => EnumStatusDTO::TODO,
        EnumStatus::InProgress => EnumStatusDTO::IN_PROGRESS,
        EnumStatus::Completed => EnumStatusDTO::COMPLETED,
    }
}

fn to_tarefa_dto(tarefa: tarefas::Model) -> TarefaDTO {
    TarefaDTO {
        id: tarefa.id,
        usuarioid: tarefa.usuario_id,
        titulo: tarefa.titulo.clone(),
        descricao: tarefa.descricao.clone(),
        status: to_enum_status_dto(tarefa.status.clone()),
        createdat: tarefa.created_at,
        updatedat: tarefa.updated_at,
        praso: tarefa.praso,
    }
}