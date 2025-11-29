pub mod upgrade;
pub mod migration;

use axum::Json;
use serde_json::{json, Value};

pub async fn health() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "upgrade-manager-backend",
        "version": "0.1.0"
    }))
}
