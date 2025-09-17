use std::fmt;

#[derive(Debug)]
pub enum DbErrPort {
  NotFound,
  ConnectionError,
  Unknown(String),
}

impl fmt::Display for DbErrPort {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      DbErrPort::NotFound => write!(f, "Registro não encontrado"),
      DbErrPort::ConnectionError => write!(f, "Erro de conexão com o banco"),
      DbErrPort::Unknown(msg) => write!(f, "Erro desconhecido: {}", msg),
    }
  }
}
