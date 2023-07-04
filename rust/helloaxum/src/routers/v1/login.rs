use axum::{Router};
use axum::routing::{post};
use serde_json::{json, Value};
use crate::dtos::login::LoginReq;
use crate::error::{Result};
use crate::utils::JsonParam;

async fn login_req(JsonParam(login_req): JsonParam<LoginReq>) -> Result<axum::Json<Value>> {
    let body = axum::Json(json!({
        "result" : {
            "success": true
        }
    }));
    Ok(body)
}

pub fn router() -> Router {
    Router::new()
        .route("/", post(login_req))
}