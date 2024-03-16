use axum::{response::IntoResponse, Json};

pub async fn find_by_all() -> impl IntoResponse {
  Json(vec![
    "Create a new todo".to_string(),
    "Read all todos".to_string(),
    "Read a todo by id".to_string(),
    "Update a todo by id".to_string(),
    "Delete a todo by id".to_string(),
  ])
}
