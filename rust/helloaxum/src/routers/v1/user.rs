use axum::extract::Path;
use axum::{Json, Router};
use axum::routing::get;
use hyper::StatusCode;
use crate::dtos::user::{User, Users};
use crate::error::{ Error, Result };
use uuid::Uuid;


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
                }
            ]
        })
    )
}

async fn create_user() -> Result<(StatusCode, Json<User>)> {
    Err(Error::NotImplemented)
}

async fn user_detail(Path(id):Path<u32>) -> Result<(StatusCode,String)> {
    //Ok( (StatusCode::CREATED, id.to_string()))
    Err(Error::NotImplemented)
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(users).post(create_user))
        .route("/:id", get(user_detail))
}