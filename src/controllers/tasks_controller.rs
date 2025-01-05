use crate::config::app_context::AppContext;
use crate::config::format;
use crate::config::routes_config::Routes;
use crate::entity::prelude::Task;
use crate::entity::task;
use crate::Result;
use axum::extract::State;
use axum::routing::get;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use axum_core::response::Response;
use sea_orm::{ActiveModelTrait, DeleteResult, EntityTrait, ModelTrait, Set};
use serde::Deserialize;
use serde_json::json;

pub fn routes() -> Routes {
  Routes::new()
    .prefix("/api/tasks")
    .add("/", get(get_tasks).post(create_task))
    .add(
      "/{id}",
      get(get_task).patch(update_task).delete(delete_taks),
    )
}

pub async fn get_tasks(State(ctx): State<AppContext>) -> Result<Response> {
  // let db = db_connection().await.unwrap();

  let tasks = Task::find().all(&ctx.db).await?;

  // Convert tasks to a Vec<serde_json::Value>
  let tasks = tasks
    .into_iter()
    .map(|task| {
      serde_json::json!({
        "id": task.id,
        "title": task.title,
        "description": task.description
      })
    })
    .collect::<Vec<serde_json::Value>>();

  format::json(tasks)
}

pub async fn get_task(State(ctx): State<AppContext>, Path(id): Path<u16>) -> impl IntoResponse {
  // let db = db_connection().await.unwrap();

  let task = Task::find_by_id(id).one(&ctx.db).await.unwrap();

  let task = task
    .into_iter()
    .map(|task| {
      serde_json::json!({
        "id": task.id,
        "title": task.title,
        "description": task.description
      })
    })
    .collect::<Vec<serde_json::Value>>();

  Json(serde_json::json!(task))
}

#[derive(Debug, Deserialize)]
pub struct CreateTask {
  title: String,
  description: String,
}

pub async fn create_task(
  State(ctx): State<AppContext>,
  Json(body): Json<CreateTask>,
) -> impl IntoResponse {
  // let db = db_connection().await.unwrap();

  let task = task::ActiveModel {
    title: Set(String::from(body.title)),
    description: Set(String::from(body.description)),
    ..Default::default() // all other attributes are `NotSet`
  };

  let task: task::Model = task.insert(&ctx.db).await.unwrap();

  (
    StatusCode::CREATED,
    Json(serde_json::json!({
      "id": task.id,
      "title": task.title,
    })),
  )
}

#[derive(Debug, Deserialize)]
pub struct UpdateTask {
  title: String,
  description: String,
}

pub async fn update_task(
  State(ctx): State<AppContext>,
  Path(id): Path<u16>,
  Json(body): Json<UpdateTask>,
) -> Result<impl IntoResponse, Json<serde_json::Value>> {
  println!("ID: {}", id);

  println!("Body: {:?}", body);

  // let db = db_connection().await.unwrap();

  // UPDATE title of Post by ID
  let task: Option<task::Model> = Task::find_by_id(id).one(&ctx.db).await.unwrap();

  // transform Option<task::Model> to task::ActiveModel
  let mut task: task::ActiveModel = task.unwrap().into();

  task.title = Set(body.title.to_owned());
  task.description = Set(body.description.to_owned());

  let task = task.update(&ctx.db).await.unwrap();

  println!("Post updated with ID: {}, TITLE: {}", task.id, task.title);

  Ok(Json(json!({ "message": "Task updated!" })))
}

pub async fn delete_taks(
  State(ctx): State<AppContext>,
  Path(id): Path<u16>,
) -> Result<impl IntoResponse, StatusCode> {
  // let db = db_connection().await.unwrap();

  // DELETE Post by ID
  let task = Task::find_by_id(id).one(&ctx.db).await.unwrap();
  let task = task.unwrap();

  let res: DeleteResult = task.delete(&ctx.db).await.unwrap();
  assert_eq!(res.rows_affected, 1);

  println!("Post deleted: {:?}", res);

  Ok(Json(json! ({
    "msg": "Task deleted! 🦀",
  })))
}
