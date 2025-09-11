use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(non_camel_case_types)]
#[derive(Debug, serde::Serialize)]
pub enum EnumStatusDTO {
  TODO,
  IN_PROGRESS,
  COMPLETED,
}

#[derive(Serialize)]
pub struct TarefaDTO {
  // entidade que descreve o dado no banco
  pub id: Uuid,
  pub usuarioid: Uuid,
  pub titulo: String,
  pub descricao: String,
  pub status: EnumStatusDTO,
  pub createdat: NaiveDateTime,
  pub updatedat: NaiveDateTime,
  pub praso: NaiveDate,
}

#[derive(Deserialize)]
pub struct TodoInputDto {
  // objeto que representa a entrada para criação de uma tarefa
  pub titulo: String,
  pub descricao: String,
  pub praso: NaiveDate,
}

#[derive(Deserialize)]
pub struct TodoUpdateInputDto {
  // objeto que representa a entrada para criação de uma tarefa
  pub titulo: Option<String>,
  pub descricao: Option<String>,
  pub praso: Option<NaiveDate>,
}
