use axum::{Router};
use axum::routing::post;
use serde_json::{json, Value};
use tracing::debug;

use crate::customext;
use crate::customext::FallbackCustomMethodNotAllowed;
use crate::dtos::login::{LoginPayload};
use crate::error::{Error, Result};

async fn login_req(customext::Json(login_payload): customext::Json<LoginPayload>)
                   -> Result<axum::Json<Value>> {
    debug!("{login_payload:?}");
    let body = axum::Json(json!({
        "result" : {
            "success": true
        }
    }));
    Ok(body)
}

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/login", post(login_req))
}