use crate::Cindy;
use axum::Router;

mod api;
mod error;
mod frontend;

pub use error::Error;

pub fn router() -> Router<Cindy> {
    Router::new()
        .nest("/api/v1", api::router())
        .merge(frontend::router())
}
