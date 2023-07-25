use crate::Cindy;
use axum::Router;

mod file;
mod query;
mod tags;

pub fn router() -> Router<Cindy> {
    Router::new()
        .nest("/file", file::router())
        .nest("/query", query::router())
        .nest("/tags", tags::router())
}
