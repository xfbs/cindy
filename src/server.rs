use crate::Cindy;
use axum::Router;

mod api;
mod frontend;

pub fn router() -> Router<Cindy> {
    Router::new()
        .nest("/api/v1", api::router())
        .merge(frontend::router())
}
