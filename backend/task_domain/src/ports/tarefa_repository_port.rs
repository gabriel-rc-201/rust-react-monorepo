use uuid::Uuid;

use crate::{
  commons::err::DbErrPort,
  models::tarefa::{CriarTarefaModel, TarefaModel},
};

pub trait ITarefaRepositoryPort: Send + Sync {
  fn get_tarefas(
    &self,
    usuario_id: Uuid,
  ) -> impl std::future::Future<Output = Result<Vec<TarefaModel>, DbErrPort>> + Send;
  fn criar_tarefa(
    &self,
    nova_tarefa: CriarTarefaModel,
  ) -> impl std::future::Future<Output = Result<TarefaModel, DbErrPort>> + Send;
  fn find_by_id(
    &self,
    tarefa_id: Uuid,
  ) -> impl std::future::Future<Output = Result<std::option::Option<TarefaModel>, DbErrPort>> + Send;
  fn atualizar_tarefa(
    &self,
    tarefa_atualizada: TarefaModel,
  ) -> impl std::future::Future<Output = Result<TarefaModel, DbErrPort>> + Send;
  fn deletar_tarefa(
    &self,
    id: Uuid,
  ) -> impl std::future::Future<Output = Result<(), DbErrPort>> + Send;
}
