use crate::Cindy;
use axum::Router;

mod api;
mod error;
mod frontend;
mod query;

use error::Error;
use query::Query;

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
