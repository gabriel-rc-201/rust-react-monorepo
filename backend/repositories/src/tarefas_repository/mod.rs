use uuid::Uuid;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder
};
use crate::entities::{
  prelude::Tarefas, tarefas
};

#[derive(Clone)]
pub struct TarefasRepository {
  db_conn: DatabaseConnection
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
}