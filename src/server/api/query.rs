use crate::Cindy;
use axum::{routing::get, Router};

async fn query() -> String {
    String::default()
}

pub fn router() -> Router<Cindy> {
    Router::new().route("/", get(query))
}
