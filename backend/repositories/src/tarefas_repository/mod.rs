use uuid::Uuid;

use crate::entities::{prelude::Tarefas, tarefas};
use sea_orm::{
  ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, QueryFilter,
  QueryOrder,
};

#[derive(Clone)]
pub struct TarefasRepository {
  db_conn: DatabaseConnection,
}

impl TarefasRepository {
  pub fn new(db_conn: DatabaseConnection) -> Self {
    TarefasRepository { db_conn: (db_conn) }
  }

  pub async fn get_tarefas(&self, usuario_id: Uuid) -> Result<Vec<tarefas::Model>, sea_orm::DbErr> {
    Tarefas::find()
      .filter(tarefas::Column::UsuarioId.eq(usuario_id))
      .order_by_asc(tarefas::Column::Praso)
      .all(&self.db_conn)
      .await
  }

  pub async fn criar_tarefa(
    &self,
    nova_tarefa: tarefas::ActiveModel,
  ) -> Result<tarefas::Model, sea_orm::DbErr> {
    nova_tarefa.insert(&self.db_conn).await
  }

  pub async fn find_by_id(
    &self,
    tarefa_id: Uuid,
  ) -> Result<std::option::Option<tarefas::Model>, sea_orm::DbErr> {
    Tarefas::find_by_id(tarefa_id).one(&self.db_conn).await
  }

  pub async fn atualizar_tarefa(
    &self,
    tarefa_atualizada: tarefas::ActiveModel,
  ) -> Result<tarefas::Model, sea_orm::DbErr> {
    tarefa_atualizada.update(&self.db_conn).await
  }

  pub async fn deletar_tarefa(&self, id: Uuid) -> Result<DeleteResult, sea_orm::DbErr> {
    Tarefas::delete_by_id(id).exec(&self.db_conn).await
  }
}
