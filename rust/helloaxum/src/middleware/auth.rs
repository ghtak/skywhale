use axum::http::Request;
use axum::middleware::Next;
use hyper::Body;
use tower_cookies::Cookies;
use crate::error::{Error, Result};

pub async fn cookie_auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next<Body>
) -> Result<axum::response::Response> {
    let auth_token = cookies.get("auth-token").map(|c| c.value().to_string());
    auth_token.ok_or(Error::AuthTokenNotExist)?;
    Ok(next.run(req).await)
}