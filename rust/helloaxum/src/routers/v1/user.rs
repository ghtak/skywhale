use axum::{Json, Router};
use axum::routing::get;
use hyper::StatusCode;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::customext;
use crate::customext::FallbackCustomMethodNotAllowed;
use crate::dtos::user::{User, Users};
use crate::error::{Error, Result};

async fn users() -> Result<Json<Users>> {
    Ok(
        Json::from(Users {
            count: 2,
            users: vec![
                User {
                    id: 1000,
                    name: String::from("s"),
                    uuid: Uuid::default(),
                },
                User {
                    id: 1001,
                    name: String::from("se"),
                    uuid: Uuid::default(),
                },
            ],
        })
    )
}

async fn create_user() -> Result<(StatusCode, Json<User>)> {
    Err(Error::NotImplemented)
}

async fn user_detail(customext::Path(id): customext::Path<u32>) -> Result<(StatusCode, String)> {
    //Ok( (StatusCode::CREATED, id.to_string()))
    Err(Error::NotImplemented)
}

async fn path_test(customext::Path(params): customext::Path<Param>) -> Result<&'static str> {
    Err(Error::NotImplemented)
}

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/user",
               get(users)
                   .post(create_user))
        .route("/api/v1/user/:id", get(user_detail))
        .route("/api/v1/user/path_test/:a_id/:b_id", get(path_test))
}

#[derive(Debug, Deserialize, Serialize)]
struct Param {
    a_id: u32,
    b_id: u32,
}