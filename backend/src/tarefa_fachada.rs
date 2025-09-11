use axum::http::StatusCode;
use repositories::entities::sea_orm_active_enums::EnumStatus;
use repositories::{entities::tarefas, tarefas_repository::TarefasRepository};
use sea_orm::{ActiveValue, DatabaseConnection, IntoActiveModel};
use serde_json::json;
use uuid::Uuid;

use crate::todo_dtos::{EnumStatusDTO, TarefaDTO, TodoInputDto, TodoUpdateInputDto};

const USUARIO_ID_MOCK: &str = "0f8d6d1f-272e-43e9-ad54-2351bdec5192";

#[derive(Clone)]
pub struct TarefaFachada {
  tarefa_repository: TarefasRepository,
}

impl TarefaFachada {
  pub fn new(db_conn: DatabaseConnection) -> Self {
    TarefaFachada {
      tarefa_repository: TarefasRepository::new(db_conn.clone()),
    }
  }

  pub async fn get_tarefas(&self) -> Result<(StatusCode, String), (StatusCode, String)> {
    let usuario_id = Uuid::parse_str(USUARIO_ID_MOCK).unwrap();

    let tarefas_db = self
      .tarefa_repository
      .get_tarefas(usuario_id)
      .await
      .map_err(|e| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          json!({"success": false, "message": e.to_string()}).to_string(),
        )
      })?;

    let tarefas_dto: Vec<TarefaDTO> = tarefas_db
      .iter()
      .map(|tarefa| to_tarefa_dto(tarefa.clone()))
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

  pub async fn criar_tarefa(
    &self,
    nova_tarefa: TodoInputDto,
  ) -> Result<(StatusCode, String), (StatusCode, String)> {
    let usuario_id: Uuid = Uuid::parse_str(USUARIO_ID_MOCK).unwrap(); // temporário até fazer a autenticação

    let tarefa_criada = tarefas::ActiveModel {
      usuario_id: ActiveValue::set(usuario_id),
      titulo: ActiveValue::set(nova_tarefa.titulo),
      descricao: ActiveValue::set(nova_tarefa.descricao),
      praso: ActiveValue::set(nova_tarefa.praso),
      ..Default::default()
    };

    let tarefa_model = self
      .tarefa_repository
      .criar_tarefa(tarefa_criada)
      .await
      .map_err(|e| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          json!({"success": false, "message": e.to_string()}).to_string(),
        )
      })?;

    let tarefa_dto = to_tarefa_dto(tarefa_model);
    Ok((
      StatusCode::CREATED,
      json!({"success": true, "data": tarefa_dto}).to_string(),
    ))
  }

  pub async fn atualizar_tarefa(
    &self,
    id: Uuid,
    tarefa: TodoUpdateInputDto,
  ) -> Result<(StatusCode, String), (StatusCode, String)> {
    let tarefa_bd: tarefas::Model = self
      .tarefa_repository
      .find_by_id(id)
      .await
      .map_err(|e| {
        (
          StatusCode::NOT_FOUND,
          json!({"success": false, "message": e.to_string()}).to_string(),
        )
      })?
      .expect("tarefa não encontrada");

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

    let tarefa_atualizada = self
      .tarefa_repository
      .atualizar_tarefa(tarefa_active_model)
      .await
      .map_err(|e| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          json!({"success": false, "message": e.to_string()}).to_string(),
        )
      })?;

    Ok((
      StatusCode::OK,
      json!({"success": true, "data": to_tarefa_dto(tarefa_atualizada)}).to_string(),
    ))
  }

  pub async fn deletar_tarefa(
    &self,
    id: Uuid,
  ) -> Result<(StatusCode, String), (StatusCode, String)> {
    self
      .tarefa_repository
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
