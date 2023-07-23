use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn create_router() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> Json<Value> {
    Json(json!({
        "endpoints": [
            {
                "path": "/",
                "methods": ["GET"],
                "description": "Returns this list of endpoints"
            },
        ]
    }))
}
