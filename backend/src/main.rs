use std::sync::Arc;
mod tarefa_fachada;
mod todo_dtos;
use crate::{
  tarefa_fachada::TarefaFachada,
  todo_dtos::{TodoInputDto, TodoUpdateInputDto},
};

use axum::{
  Json, Router,
  extract::{Path, State},
  routing::{get, post, put},
  serve,
};
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;
use uuid::Uuid;

#[tokio::main]
async fn main() {
  // carregando variáveis de ambiente
  dotenv().expect("Unable to access .env file");
  let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("localhost:3000".to_owned());
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found on .env file");

  // estabelecendo conexão com o banco de dados
  let mut opt = ConnectOptions::new(database_url);
  opt.max_connections(16);
  let db_conn = Database::connect(opt)
    .await
    .expect("Failed to connect to the database");

  let tarefa_fachada = TarefaFachada::new(db_conn.clone());

  let app_state = AppState::new(Arc::new(tarefa_fachada));

  let app = Router::new()
    .route("/", get(|| async { "Hello World!" }))
    .route(
      "/todos",
      get(|State(app_state): State<AppState>| async move {
        app_state.tarefa_fachada.get_tarefas().await
      }),
    )
    .route(
      "/todo",
      post(
        |State(app_state): State<AppState>, Json(nova_tarefa): Json<TodoInputDto>| async move {
          app_state.tarefa_fachada.criar_tarefa(nova_tarefa).await
        },
      ),
    )
    .route(
      "/todo/{id}",
      put(
        |State(app_state): State<AppState>,
         Path(id): Path<Uuid>,
         Json(tarefa_dto): Json<TodoUpdateInputDto>| async move {
          app_state
            .tarefa_fachada
            .atualizar_tarefa(id, tarefa_dto)
            .await
        },
      )
      .delete(
        |State(app_state): State<AppState>, Path(id): Path<Uuid>| async move {
          app_state.tarefa_fachada.deletar_tarefa(id).await
        },
      ),
    )
    .with_state(app_state);

  let listener = TcpListener::bind(server_address)
    .await
    .expect("Failed to bind to address");

  let addr = listener.local_addr().expect("Failed to get local address");

  println!("Server is running on {}", addr);

  serve(listener, app)
    .await
    .expect("Error serving application");
}

#[derive(Clone)]
struct AppState {
  tarefa_fachada: Arc<TarefaFachada>,
}

impl AppState {
  fn new(tarefa_fachada: Arc<TarefaFachada>) -> Self {
    AppState { tarefa_fachada }
  }
}
