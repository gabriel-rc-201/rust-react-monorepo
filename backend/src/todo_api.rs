use crate::todo_dtos::{EnumStatusDTO, TarefaDTO, TodoInputDto, TodoUpdateInputDto};
use axum::{
  Json, Router,
  extract::{Path, State},
  http::StatusCode,
  routing::{get, post, put},
};
use repositories::tarefas_repository::TarefasRepository;
use sea_orm::DatabaseConnection;
use serde_json::json;
use std::sync::Arc;
use task_domain::{
  fachada::tarefa_fachada::{AtualizarTarefaUseCase, CriarTarefaUseCase, TarefaFachada},
  models::tarefa::{EnumStatusModel, TarefaModel},
};
use uuid::Uuid;

pub struct TodoApi {
  app_state: AppState,
}

impl TodoApi {
  pub fn new(db_conn: DatabaseConnection) -> Self {
    let tarefa_repository = TarefasRepository::new(db_conn.clone());
    let tarefa_fachada: TarefaFachada<TarefasRepository> =
      TarefaFachada::new(tarefa_repository.clone());

    TodoApi {
      app_state: AppState::new(Arc::new(tarefa_fachada)),
    }
  }

  pub fn routes(&self) -> Router {
    Router::new()
      .route("/", get(|| async { "Hello World!" }))
      .route("/todos", get(get_tarefas))
      .route("/todo", post(post_tarefa))
      .route("/todo/{id}", put(atualizar_tarefa).delete(deletar_tarefa))
      .with_state(self.app_state.clone())
  }
}

#[derive(Clone)]
pub struct AppState {
  tarefa_fachada: Arc<TarefaFachada<TarefasRepository>>,
}

impl AppState {
  pub fn new(tarefa_fachada: Arc<TarefaFachada<TarefasRepository>>) -> Self {
    AppState { tarefa_fachada }
  }
}

const USUARIO_ID_MOCK: &str = "0f8d6d1f-272e-43e9-ad54-2351bdec5192";

async fn get_tarefas(
  State(app_state): State<AppState>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let usuario_id = Uuid::parse_str(USUARIO_ID_MOCK).unwrap();
  let tarefas_model = app_state
    .tarefa_fachada
    .get_tarefas(usuario_id)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": e.to_string()}).to_string(),
      )
    })?;

  let tarefas_dto: Vec<TarefaDTO> = tarefas_model
    .iter()
    .map(|tarefa| to_tarefa_dto(tarefa))
    .collect();

  Ok((
    StatusCode::OK,
    json!({
        "success": true,
        "data": tarefas_dto
    })
    .to_string(),
  ))
}

async fn post_tarefa(
  State(app_state): State<AppState>,
  Json(nova_tarefa): Json<TodoInputDto>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let use_case = CriarTarefaUseCase {
    usuario_id: Uuid::parse_str(USUARIO_ID_MOCK).unwrap(),
    titulo: nova_tarefa.titulo,
    descricao: nova_tarefa.descricao,
    praso: nova_tarefa.praso,
  };

  let tarefa_model = app_state
    .tarefa_fachada
    .criar_tarefa(use_case)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": e.to_string()}).to_string(),
      )
    })?;

  let tarefa_dto = to_tarefa_dto(&tarefa_model);
  Ok((
    StatusCode::CREATED,
    json!({"success": true, "data": tarefa_dto}).to_string(),
  ))
}

async fn atualizar_tarefa(
  State(app_state): State<AppState>,
  Path(id): Path<Uuid>,
  Json(tarefa_dto): Json<TodoUpdateInputDto>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let use_case = AtualizarTarefaUseCase {
    tarefa_id: id,
    titulo: tarefa_dto.titulo,
    descricao: tarefa_dto.descricao,
    praso: tarefa_dto.praso,
  };

  let tarefa_model = app_state
    .tarefa_fachada
    .atualizar_tarefa(use_case)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": e.to_string()}).to_string(),
      )
    })?;

  Ok((
    StatusCode::OK,
    json!({"success": true, "data": to_tarefa_dto(&tarefa_model)}).to_string(),
  ))
}

async fn deletar_tarefa(
  State(app_state): State<AppState>,
  Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  app_state
    .tarefa_fachada
    .deletar_tarefa(id)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": e.to_string()}).to_string(),
      )
    })?;

  Ok((
    StatusCode::OK,
    json!({"success": true, "message": "tarefa deletada com sucesso"}).to_string(),
  ))
}

fn to_enum_status_dto(status: EnumStatusModel) -> EnumStatusDTO {
  match status {
    EnumStatusModel::TODO => EnumStatusDTO::TODO,
    EnumStatusModel::IN_PROGRESS => EnumStatusDTO::IN_PROGRESS,
    EnumStatusModel::COMPLETED => EnumStatusDTO::COMPLETED,
  }
}

fn to_tarefa_dto(tarefa: &TarefaModel) -> TarefaDTO {
  TarefaDTO {
    id: tarefa.id,
    usuarioid: tarefa.usuario_id,
    titulo: tarefa.titulo.clone(),
    descricao: tarefa.descricao.clone(),
    status: to_enum_status_dto(tarefa.status),
    createdat: tarefa.created_at,
    updatedat: tarefa.updated_at,
    praso: tarefa.praso,
  }
}
