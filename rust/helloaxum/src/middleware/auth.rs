use axum::extract::FromRequestParts;
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::RequestPartsExt;
use axum::async_trait;
use hyper::Body;
use tower_cookies::Cookies;
use crate::error::{Error, Result};

pub async fn cookie_auth(
    //cookies: Cookies,
    session: SessionSample,
    req: Request<Body>,
    next: Next<Body>
) -> Result<axum::response::Response> {
    //let auth_token = cookies.get("auth-token").map(|c| c.value().to_string());
    //auth_token.ok_or(Error::AuthTokenNotExist)?;
    Ok(next.run(req).await)
}

#[derive(Debug)]
pub struct SessionSample{
    auth_token : String,
}

#[async_trait]
impl<S> FromRequestParts<S> for SessionSample
where
     S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(
        parts: &mut Parts, state: &S
    ) -> core::result::Result<Self, Self::Rejection> {
        let cookies = parts.extract::<Cookies>().await.unwrap();
        let auth_token = cookies.get("auth-token").map(|c| c.value().to_string());
        let auth_token = auth_token.ok_or(Error::AuthTokenNotExist)?;
        Ok(SessionSample{ auth_token })
    }
}