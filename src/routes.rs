use std::i32;

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

use crate::{models::TodoItem, AppState};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/create", post(hanlde_create_todo))
        .route("/todos", get(hanldle_list_todos))
        .route("/todos/:id", get(hanldle_list_todo_item))
        .with_state(app_state)
}

async fn index() -> Json<Value> {
    Json(json!({
        "endpoints": [
            {
                "path": "/",
                "methods": ["GET"],
                "description": "Returns this list of endpoints"
            },
            {
                "path": "/create",
                "methods": ["POST"],
                "description": "Creates a new todo item"
            },
            {
                "path": "/todos",
                "methods": ["GET"],
                "description": "Returns a list of all todo items"
            },
            {
                "path": "/todos/{id}",
                "methods": ["GET"],
                "description": "Returns a single todo item"
            },
            {
                "path": "/update/{id}",
                "methods": ["PUT"],
                "description": "Updates a single todo item"
            },
            {
                "path": "/delete/{id}",
                "methods": ["DELETE"],
                "description": "Deletes a single todo item"
            }
        ]
    }))
}

async fn hanlde_create_todo(
    State(app): State<AppState>,
    Json(payload): Json<TodoItem>,
) -> Json<Value> {
    let result = app.db.create_task(&payload).await;

    match result {
        Ok(_) => {
            tracing::info!("Created new todo item: {:?}", payload);
            Json(json!({ "success": true }))
        }
        Err(e) => {
            tracing::error!("Failed to create new todo item: {:?}", e);
            Json(json!({ "success": false, "error": e.to_string() }))
        }
    }
}

async fn hanldle_list_todos(State(app): State<AppState>) -> Json<Value> {
    let result = app.db.list_tasks().await;

    match result {
        Ok(tasks) => {
            tracing::info!("Listed all todo items");
            Json(json!({ "success": true, "tasks": tasks }))
        }
        Err(e) => {
            tracing::error!("Failed to list all todo items: {:?}", e);
            Json(json!({ "success": false, "error": e.to_string() }))
        }
    }
}

async fn hanldle_list_todo_item(State(app): State<AppState>, Path(id): Path<i32>) -> Json<Value> {
    let result: Result<Vec<crate::models::TodoList>, diesel::result::Error> =
        app.db.list_task(id).await;

    match result {
        Ok(tasks) => {
            tracing::info!("Listed all todo items");
            Json(json!({ "success": true, "tasks": tasks }))
        }
        Err(e) => {
            tracing::error!("Failed to list all todo items: {:?}", e);
            Json(json!({ "success": false, "error": e.to_string() }))
        }
    }
}
