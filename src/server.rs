use crate::Cindy;
use axum::Router;

mod api;
mod error;
mod frontend;

use error::Error;

fn router() -> Router<Cindy> {
    Router::new()
        .nest("/api/v1", api::router())
        .merge(frontend::router())
}

impl Cindy {
    pub fn router(&self) -> Router {
        router().with_state(self.clone())
    }
}
