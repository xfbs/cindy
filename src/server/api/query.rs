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
    spawn_blocking(move || database.query_hashes(&mut query.query.iter()))
        .await?
        .map(Json)
        .map_err(Into::into)
}

async fn query_tag_create(
    State(cindy): State<Cindy>,
    Json(request): Json<QueryTagCreate<String>>,
) -> Result<(), Error> {
    let database = cindy.database().await;
    spawn_blocking(move || {
        database.query_tag_add(&mut request.query.iter(), &request.name, &request.value)
    })
    .await?
    .map_err(Into::into)
}

async fn query_tag_delete(
    State(cindy): State<Cindy>,
    Query(query): Query<QueryTagRemove<String>>,
) -> Result<(), Error> {
    let database = cindy.database().await;
    spawn_blocking(move || {
        database.query_tag_remove(
            &mut query.query.iter(),
            query.name.as_deref(),
            query.value.as_deref(),
        )
    })
    .await?
    .map_err(Into::into)
}

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
