use crate::{hash::ArcHash, server::Error, Cindy};
use axum::{
    body::StreamBody,
    extract::{Path, Query, State},
    http::{header, HeaderValue},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use cindy_common::api::*;
use std::path::PathBuf;
use tokio::{fs::File, task::spawn_blocking};
use tokio_util::io::ReaderStream;

async fn stream_file(
    State(cindy): State<Cindy>,
    Path(hash): Path<ArcHash>,
) -> Result<impl IntoResponse, Error> {
    // get filenames
    let database = cindy.database().await;
    let hash_clone = hash.clone();
    let tags =
        spawn_blocking(move || database.hash_tags(&hash_clone, Some("filename"), None)).await??;

    // pick content type based on whatever filename the file is tagged with,
    // defaulting to application/octet-stream.
    let content_type = tags
        .into_iter()
        .map(|name| PathBuf::from(name.value()))
        .find(|path| path.extension().is_some())
        .and_then(|path| {
            mime_guess::from_path(path)
                .first_raw()
                .map(HeaderValue::from_static)
        })
        .unwrap_or_else(|| HeaderValue::from_str(mime::APPLICATION_OCTET_STREAM.as_ref()).unwrap());

    // get path to file on disk
    let path = cindy.hash_path(&hash);

    // open file and turn into stream
    let file = File::open(&path).await?;
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    // return headers and stream
    let headers = [(header::CONTENT_TYPE, content_type)];
    Ok((headers, body))
}

async fn file_tags(
    State(cindy): State<Cindy>,
    Path(hash): Path<ArcHash>,
    Query(query): Query<TagQuery<String>>,
) -> Result<impl IntoResponse, Error> {
    // get filenames
    let database = cindy.database().await;
    let tags = spawn_blocking(move || {
        database.hash_tags(&hash, query.name.as_deref(), query.value.as_deref())
    })
    .await??;

    Ok(Json(tags))
}

async fn file_tag_create(
    State(cindy): State<Cindy>,
    Path(hash): Path<ArcHash>,
    Json(request): Json<FileTagCreateBody<'static>>,
) -> Result<(), Error> {
    // get filenames
    let database = cindy.database().await;
    spawn_blocking(move || database.hash_tag_add(&hash, &request.name, &request.value)).await??;

    Ok(())
}

async fn file_tag_delete(
    State(cindy): State<Cindy>,
    Query(query): Query<TagQuery<String>>,
    Path(hash): Path<ArcHash>,
) -> Result<(), Error> {
    // get filenames
    let database = cindy.database().await;
    let hash_clone = hash.clone();
    spawn_blocking(move || {
        database.hash_tag_remove(&hash_clone, query.name.as_deref(), query.value.as_deref())
    })
    .await??;

    Ok(())
}

async fn file_labels(
    State(cindy): State<Cindy>,
    Path(hash): Path<ArcHash>,
    Query(query): Query<TagQuery<String>>,
) -> Result<impl IntoResponse, Error> {
    // get filenames
    let database = cindy.database().await;
    let labels = spawn_blocking(move || {
        database.label_get(
            Some(&hash),
            query.name.as_deref(),
            query.value.as_deref(),
            None,
        )
    })
    .await??;

    Ok(Json(labels))
}

async fn file_label_delete() {}

async fn file_label_create() {}

pub fn router() -> Router<Cindy> {
    Router::new()
        .route("/:hash", get(stream_file))
        .route(
            "/:hash/tags",
            get(file_tags).delete(file_tag_delete).post(file_tag_create),
        )
        .route(
            "/:hash/labels",
            get(file_labels)
                .delete(file_label_delete)
                .post(file_label_create),
        )
}
