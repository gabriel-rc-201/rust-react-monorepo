use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
  commons::err::DbErrPort,
  models::tarefa::{CriarTarefaModel, TarefaModel},
  ports::tarefa_repository_port::ITarefaRepositoryPort,
};

#[derive(Clone)]
pub struct TarefaFachada<R: ITarefaRepositoryPort> {
  tarefa_repository: R,
}

impl<R: ITarefaRepositoryPort> TarefaFachada<R> {
  pub fn new(tarefa_repository: R) -> Self {
    TarefaFachada { tarefa_repository }
  }

  pub async fn get_tarefas(&self, usuario_id: Uuid) -> Result<Vec<TarefaModel>, DbErrPort> {
    self.tarefa_repository.get_tarefas(usuario_id).await
  }

  pub async fn criar_tarefa(
    &self,
    criar_tareva_use_case: CriarTarefaUseCase,
  ) -> Result<TarefaModel, DbErrPort> {
    let tarefa_criada = CriarTarefaModel {
      usuario_id: criar_tareva_use_case.usuario_id,
      titulo: criar_tareva_use_case.titulo,
      descricao: criar_tareva_use_case.descricao,
      praso: criar_tareva_use_case.praso,
    };

    self.tarefa_repository.criar_tarefa(tarefa_criada).await
  }

  pub async fn atualizar_tarefa(
    &self,
    atualizar_tarefa_use_case: AtualizarTarefaUseCase,
  ) -> Result<TarefaModel, DbErrPort> {
    let mut tarefa_model: TarefaModel = self
      .tarefa_repository
      .find_by_id(atualizar_tarefa_use_case.tarefa_id)
      .await?
      .expect("tarefa não encontrada");

    if let Some(titulo) = atualizar_tarefa_use_case.titulo {
      tarefa_model.titulo = titulo;
    }
    if let Some(descricao) = atualizar_tarefa_use_case.descricao {
      tarefa_model.descricao = descricao;
    }
    if let Some(praso) = atualizar_tarefa_use_case.praso {
      tarefa_model.praso = praso;
    }

    self.tarefa_repository.atualizar_tarefa(tarefa_model).await
  }

  pub async fn deletar_tarefa(&self, id: Uuid) -> Result<(), DbErrPort> {
    self.tarefa_repository.deletar_tarefa(id).await
  }
}

pub struct CriarTarefaUseCase {
  pub usuario_id: Uuid,
  pub titulo: String,
  pub descricao: String,
  pub praso: NaiveDate,
}

pub struct AtualizarTarefaUseCase {
  pub tarefa_id: Uuid,
  pub titulo: Option<String>,
  pub descricao: Option<String>,
  pub praso: Option<NaiveDate>,
}
