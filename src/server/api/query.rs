use crate::{common::api::FileQuery, hash::BoxHash, server::Error, Cindy};
use axum::{extract::State, routing::get, Json, Router};
use serde_qs::axum::QsQuery as Query;
use std::collections::BTreeSet;
use tokio::task::spawn_blocking;

async fn query(
    State(cindy): State<Cindy>,
    Query(query): Query<FileQuery<'static>>,
) -> Result<Json<BTreeSet<BoxHash>>, Error> {
    let database = cindy.database().await;
    spawn_blocking(move || database.hash_query(&mut query.query.iter()))
        .await?
        .map(Json)
        .map_err(Into::into)
}

pub fn router() -> Router<Cindy> {
    Router::new().route("/", get(query))
}