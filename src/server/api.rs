use crate::{server::Error, Cindy};
use axum::Router;

mod file;
mod query;
mod tags;

async fn not_found() -> Error {
    Error::NotFound
}

pub fn router() -> Router<Cindy> {
    Router::new()
        .nest("/file", file::router())
        .nest("/query", query::router())
        .merge(tags::router())
        .fallback(not_found)
}
