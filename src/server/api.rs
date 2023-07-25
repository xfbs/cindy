use crate::Cindy;
use axum::{
    routing::{get, post},
    Router,
};

async fn get_file() -> String {
    String::default()
}

async fn query() -> String {
    String::default()
}

pub fn router() -> Router<Cindy> {
    Router::new()
        .route("/api/v1/file/:hash", get(get_file))
        .route("/api/v1/file/:hash/tags", get(get_file))
        .route(
            "/api/v1/file/:hash/tag/:name",
            post(get_file).delete(get_file),
        )
        .route("/api/v1/query", get(query))
}
