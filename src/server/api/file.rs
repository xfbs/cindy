use crate::{hash::ArcHash, Cindy};
use axum::{
    body::StreamBody,
    extract::{Path, State},
    http::header,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

async fn stream_file(State(cindy): State<Cindy>, Path(hash): Path<ArcHash>) -> impl IntoResponse {
    let path = cindy.hash_path(&hash);
    let file = File::open(&path).await.unwrap();
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    let headers = [(header::CONTENT_TYPE, "text/html")];
    (headers, body)
}

pub fn router() -> Router<Cindy> {
    Router::new()
        .route("/:hash", get(stream_file))
        .route("/:hash/tags", get(stream_file))
        .route("/:hash/tag/:name", post(stream_file).delete(stream_file))
}
