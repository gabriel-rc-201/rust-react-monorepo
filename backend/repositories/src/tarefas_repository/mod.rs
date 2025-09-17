use uuid::Uuid;

use crate::entities::{prelude::Tarefas, sea_orm_active_enums::EnumStatus, tarefas};
use sea_orm::{
  ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
  QueryOrder,
};
use task_domain::{
  self,
  commons::err::DbErrPort,
  models::tarefa::{CriarTarefaModel, EnumStatusModel, TarefaModel},
  ports::tarefa_repository_port::ITarefaRepositoryPort,
};

#[derive(Clone)]
pub struct TarefasRepository {
  db_conn: DatabaseConnection,
}

impl TarefasRepository {
  pub fn new(db_conn: DatabaseConnection) -> Self {
    TarefasRepository { db_conn: (db_conn) }
  }
}

impl ITarefaRepositoryPort for TarefasRepository {
  async fn get_tarefas(&self, usuario_id: Uuid) -> Result<Vec<TarefaModel>, DbErrPort> {
    let result = Tarefas::find()
      .filter(tarefas::Column::UsuarioId.eq(usuario_id))
      .order_by_asc(tarefas::Column::Praso)
      .all(&self.db_conn)
      .await;

    match result {
      Ok(models) => {
        let tarefas_model = models
          .into_iter()
          .map(|tarefa| to_tarefa_model(tarefa))
          .collect();
        Ok(tarefas_model)
      }
      Err(e) => Err(to_db_err_port(e)),
    }
  }

  async fn criar_tarefa(&self, nova_tarefa: CriarTarefaModel) -> Result<TarefaModel, DbErrPort> {
    let tarefa_to_insert = to_tarefa_entity_active_model(nova_tarefa);
    let result = tarefa_to_insert.insert(&self.db_conn).await;

    match result {
      Ok(tarefa) => Ok(to_tarefa_model(tarefa)),
      Err(e) => Err(to_db_err_port(e)),
    }
  }

  async fn find_by_id(
    &self,
    tarefa_id: Uuid,
  ) -> Result<std::option::Option<TarefaModel>, DbErrPort> {
    let result = Tarefas::find_by_id(tarefa_id).one(&self.db_conn).await;

    match result {
      Ok(model) => Ok(model.map(to_tarefa_model)),
      Err(e) => Err(to_db_err_port(e)),
    }
  }

  async fn atualizar_tarefa(
    &self,
    tarefa_atualizada: TarefaModel,
  ) -> Result<TarefaModel, DbErrPort> {
    let tarefa_to_update = tarefa_model_to_tarefa_entity_active_model(tarefa_atualizada);
    let result = tarefa_to_update.update(&self.db_conn).await;

    match result {
      Ok(model) => Ok(to_tarefa_model(model)),
      Err(e) => Err(to_db_err_port(e)),
    }
  }

  async fn deletar_tarefa(&self, id: Uuid) -> Result<(), DbErrPort> {
    let result = Tarefas::delete_by_id(id).exec(&self.db_conn).await;

    match result {
      Ok(_) => Ok(()),
      Err(e) => Err(to_db_err_port(e)),
    }
  }
}

fn to_tarefa_model(tarefa_model: tarefas::Model) -> TarefaModel {
  TarefaModel {
    id: tarefa_model.id,
    usuario_id: tarefa_model.usuario_id,
    titulo: tarefa_model.titulo,
    descricao: tarefa_model.descricao,
    status: to_enum_status_model(tarefa_model.status),
    praso: tarefa_model.praso,
    created_at: tarefa_model.created_at,
    updated_at: tarefa_model.updated_at,
  }
}

fn tarefa_model_to_tarefa_entity_active_model(
  nova_tarefa_model: TarefaModel,
) -> tarefas::ActiveModel {
  tarefas::ActiveModel {
    id: ActiveValue::set(nova_tarefa_model.id),
    usuario_id: ActiveValue::set(nova_tarefa_model.usuario_id),
    titulo: ActiveValue::set(nova_tarefa_model.titulo),
    descricao: ActiveValue::set(nova_tarefa_model.descricao),
    status: ActiveValue::set(to_enum_status(nova_tarefa_model.status)),
    praso: ActiveValue::set(nova_tarefa_model.praso),
    created_at: ActiveValue::set(nova_tarefa_model.created_at),
    updated_at: ActiveValue::set(chrono::Local::now().naive_local()),
    ..Default::default()
  }
}

fn to_tarefa_entity_active_model(nova_tarefa_model: CriarTarefaModel) -> tarefas::ActiveModel {
  tarefas::ActiveModel {
    usuario_id: ActiveValue::set(nova_tarefa_model.usuario_id),
    titulo: ActiveValue::set(nova_tarefa_model.titulo),
    descricao: ActiveValue::set(nova_tarefa_model.descricao),
    praso: ActiveValue::set(nova_tarefa_model.praso),
    ..Default::default()
  }
}

fn to_enum_status_model(status: EnumStatus) -> EnumStatusModel {
  match status {
    EnumStatus::Todo => EnumStatusModel::TODO,
    EnumStatus::InProgress => EnumStatusModel::IN_PROGRESS,
    EnumStatus::Completed => EnumStatusModel::COMPLETED,
  }
}

fn to_enum_status(status: EnumStatusModel) -> EnumStatus {
  match status {
    EnumStatusModel::TODO => EnumStatus::Todo,
    EnumStatusModel::IN_PROGRESS => EnumStatus::InProgress,
    EnumStatusModel::COMPLETED => EnumStatus::Completed,
  }
}

fn to_db_err_port(err: sea_orm::DbErr) -> DbErrPort {
  match err {
    sea_orm::DbErr::Conn(_) => DbErrPort::ConnectionError,
    sea_orm::DbErr::RecordNotFound(_) => DbErrPort::NotFound,
    other => DbErrPort::Unknown(other.to_string()),
  }
}
