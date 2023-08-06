use crate::{common::api::*, hash::BoxHash, server::Error, Cindy, Tag};
use axum::{extract::State, routing::get, Json, Router};
use serde_qs::axum::QsQuery as Query;
use std::collections::BTreeSet;
use tokio::task::spawn_blocking;

async fn query(
    State(cindy): State<Cindy>,
    Query(query): Query<QueryFiles<'static>>,
) -> Result<Json<BTreeSet<BoxHash>>, Error> {
    let database = cindy.database().await;
    spawn_blocking(move || database.hash_query(&mut query.query.iter()))
        .await?
        .map(Json)
        .map_err(Into::into)
}

async fn query_tag_create() {}

async fn query_tag_delete() {}

async fn query_tags(
    State(cindy): State<Cindy>,
    Query(query): Query<QueryTags>,
) -> Result<Json<BTreeSet<Tag>>, Error> {
    let database = cindy.database().await;
    spawn_blocking(move || match query.mode {
        QueryTagsMode::Union => database.query_tag_union(
            &mut query.query.iter(),
            query.name.as_deref(),
            query.value.as_deref(),
        ),
        QueryTagsMode::Intersection => database.query_tag_intersection(
            &mut query.query.iter(),
            query.name.as_deref(),
            query.value.as_deref(),
        ),
    })
    .await?
    .map(Json)
    .map_err(Into::into)
}

pub fn router() -> Router<Cindy> {
    Router::new().route("/", get(query)).route(
        "/tags",
        get(query_tags)
            .post(query_tag_create)
            .delete(query_tag_delete),
    )
}
