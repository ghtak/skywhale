use axum::{Router};
use axum::routing::{post};
use serde_json::{json, Value};
use crate::customext;
use crate::dtos::login::LoginReq;
use crate::error::{Result};

async fn login_req(customext::Json(login_req): customext::Json<LoginReq>)
    -> Result<axum::Json<Value>> {
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