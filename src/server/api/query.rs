use crate::{common::api::*, hash::BoxHash, server::Error, Cindy};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
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

async fn query_tag_create() {}

async fn query_tag_delete() {}

async fn query_tags_union(
    State(cindy): State<Cindy>,
    Query(query): Query<QueryTagsUnion>,
) -> Result<Json<QueryTagsResponse>, Error> {
    let database = cindy.database().await;
    spawn_blocking(move || {
        let hashes = database.hash_query(&mut query.query.iter())?;
        let mut union = BTreeSet::new();
        for hash in &hashes {
            let hashes =
                database.hash_tags(&hash, query.name.as_deref(), query.value.as_deref())?;
            for hash in hashes {
                union.insert(hash);
            }
        }
        Ok(QueryTagsResponse { tags: union })
    })
    .await?
    .map(Json)
}

async fn query_tags_intersection(
    State(cindy): State<Cindy>,
    Query(query): Query<QueryTagsIntersection>,
) -> Result<Json<QueryTagsResponse>, Error> {
    let database = cindy.database().await;
    spawn_blocking(move || {
        let hashes = database.hash_query(&mut query.query.iter())?;
        let mut intersection: Option<BTreeSet<_>> = None;
        for hash in &hashes {
            let hashes =
                database.hash_tags(&hash, query.name.as_deref(), query.value.as_deref())?;
            if let Some(list) = &mut intersection {
                let difference: Vec<_> = list.difference(&hashes).cloned().collect();
                for hash in &difference {
                    list.remove(hash);
                }
            } else {
                intersection = Some(hashes);
            }
        }
        Ok(QueryTagsResponse {
            tags: intersection.unwrap_or_default(),
        })
    })
    .await?
    .map(Json)
}

pub fn router() -> Router<Cindy> {
    Router::new()
        .route("/", get(query))
        .route("/tags", post(query_tag_create).delete(query_tag_delete))
        .route("/tags/union", get(query_tags_union))
        .route("/tags/intersection", get(query_tags_intersection))
}
