use std::io;
use axum::extract::FromRequestParts;
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::RequestPartsExt;
use axum::async_trait;
use hyper::{Body, Response};
use serde_json::to_string;
use tower_cookies::Cookies;
use crate::error::{Error, Result};

pub async fn cookie_auth<B>(
    //cookies: Cookies,
    session: SessionSample,
    req: Request<B>,
    next: Next<B>
) -> Result<axum::response::Response> {
    //let auth_token = cookies.get("auth-token").map(|c| c.value().to_string());
    //auth_token.ok_or(Error::AuthTokenNotExist)?;
    Ok(next.run(req).await)
}

#[derive(Debug, Clone)]
pub struct SessionSample{
    auth_token : String,
}

impl SessionSample{
    fn new(token: String) -> Self {
        SessionSample{
            auth_token: token
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for SessionSample
where
     S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(
        parts: &mut Parts, state: &S
    ) -> Result<Self> {
        // let cookies = parts.extract::<Cookies>().await.unwrap();
        // let auth_token = cookies.get("auth-token").map(|c| c.value().to_string());
        // if let Some(token) = auth_token {
        //     Ok(SessionSample{auth_token:token})
        // } else {
        //     Err(Error::AuthTokenNotExist)
        // }
        parts.extensions
            .get::<Result<SessionSample>>()
            .ok_or(Error::AuthTokenNotExist)?
            .clone()
        // if let Ok(session) = parts.extensions.get::<Result<SessionSample>>() {
        //     Ok(session.clone())
        // }else{
        //     Err(Error::AuthTokenNotExist)
        // }
    }
}

pub async fn session_resolver<B>(
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>
) -> Result<axum::response::Response> {
    let auth_token = cookies.get("auth-token").map(|c| c.value().to_string());
    let result = match auth_token{
        Some(token) => Ok(SessionSample::new(token)),
        _ => Err(Error::AuthTokenNotExist)
    };
    req.extensions_mut().insert(result);
    // if let Some(token) = auth_token{
    //     req.extensions_mut().insert(Ok::<SessionSample, Error>(SessionSample::new(token)));
    // }
    Ok(next.run(req).await)
}