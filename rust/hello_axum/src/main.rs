use axum::{
    routing::{get, post},
    extract::Query,
    response::Html,
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/query", get(query_handler))
        .route("/users", post(create_user));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//async fn root() -> (StatusCode, &'static str) {
async fn root() -> (StatusCode, Json<Root> ) {
    let users = vec![
        User{
            id: 0,
            username: String::from("User")
        },
        User{
            id: 2,
            username: String::from("User2")
        },
    ];
    (
        StatusCode::BAD_REQUEST, 
        Json(Root{
            count: 0,
            users: users
        })
    )
}

async fn create_user(
    Json(payload) : Json<CreateUser>
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

async fn query_handler(
    Query(q): Query<QueryParam>
) -> Html<String> {
    Html(format!("<h1>Test Value: {}</h1>", q.test))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Serialize)]
struct Root{
    count: usize,
    users: Vec<User>,
}

#[derive(Deserialize)]
struct QueryParam{
    test: bool,
}