use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum EnumStatusModel {
  TODO,
  IN_PROGRESS,
  COMPLETED,
}

pub struct CriarTarefaModel {
  pub usuario_id: Uuid,
  pub titulo: String,
  pub descricao: String,
  pub praso: NaiveDate,
}

pub struct TarefaModel {
  pub id: Uuid,
  pub usuario_id: Uuid,
  pub titulo: String,
  pub descricao: String,
  pub status: EnumStatusModel,
  pub praso: NaiveDate,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}
