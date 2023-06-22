use crate::dtos::{self, sample::Sample};
use axum::{extract::Path, routing::get, Json, Router};
use hyper::StatusCode;
use uuid::Uuid;

async fn sample_str() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Sample")
}

async fn sample() -> (StatusCode, Json<Vec<Sample>>) {
    (
        StatusCode::OK,
        Json(vec![Sample {
            id: 1000,
            name: String::from("s"),
            uuid: Uuid::default(),
        }]),
    )
}

async fn sample_detail(Path(id): Path<Uuid>) -> (StatusCode, Json<dtos::sample::Sample>) {
    (
        StatusCode::OK,
        Json(Sample {
            id: 1000,
            name: String::from("s"),
            uuid: id,
        }),
    )
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(sample))
        .route("/:id", get(sample_detail))
}
