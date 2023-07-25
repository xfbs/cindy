use crate::Cindy;
use axum::{
    routing::{get, post},
    body::StreamBody,
    response::IntoResponse,
    Router,
};
use tokio_util::io::ReaderStream;

async fn stream_file() -> impl IntoResponse {
    String::default()
}

pub fn router() -> Router<Cindy> {
    Router::new()
        .route("/:hash", get(stream_file))
        .route("/:hash/tags", get(stream_file))
        .route("/:hash/tag/:name", post(stream_file).delete(stream_file))
}
