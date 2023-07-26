use crate::Cindy;
use axum::{routing::get, Router};

async fn tags() -> String {
    String::default()
}

pub fn router() -> Router<Cindy> {
    Router::new().route("/:name/:value", get(tags).post(tags).delete(tags))
}
