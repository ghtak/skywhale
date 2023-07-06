use axum::{Router};
use axum::routing::{get, post};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use crate::customext;
use crate::customext::FallbackCustomMethodNotAllowed;
use crate::dtos::login::{LoginPayload};
use crate::error::{Error, Result};

async fn login_req(
    cookies: Cookies,
    customext::Json(login_payload): customext::Json<LoginPayload>
) -> Result<axum::Json<Value>> {
    debug!("{login_payload:?}");
    let body = axum::Json(json!({
        "result" : {
            "success": true
        }
    }));
    cookies.add(Cookie::new("auth-token", "user.exp-1.sign"));
    Ok(body)
}


async fn login_test(
    cookies: Cookies
) -> Result<axum::Json<Value>> {
    let body = axum::Json(json!({
        "result" : {
            "success": true
        }
    }));
    cookies.add(Cookie::new("auth-token", "user.exp-1.sign"));
    Ok(body)
}


pub fn router() -> Router {
    Router::new()
        .route("/api/v1/login", get(login_test).post(login_req))
}