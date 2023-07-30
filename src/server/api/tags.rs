use crate::{server::Error, Cindy};
use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use cindy_common::{
    api::TagQuery,
    tag::{Tag, TagNameInfo, TagValueInfo},
};
use std::collections::BTreeMap;
use tokio::task::spawn_blocking;

async fn tag_names(
    State(cindy): State<Cindy>,
) -> Result<Json<BTreeMap<String, TagNameInfo>>, Error> {
    let database = cindy.database().await;
    spawn_blocking(move || database.tag_names().map(Json).map_err(Into::into)).await?
}

fn option_from_str(tag: &str) -> Option<&str> {
    match tag {
        "*" => None,
        other => Some(other),
    }
}

async fn tag_list(
    State(cindy): State<Cindy>,
    Query(query): Query<TagQuery<String>>,
) -> Result<Json<BTreeMap<Tag, TagValueInfo>>, Error> {
    let database = cindy.database().await;
    spawn_blocking(move || {
        database
            .tag_list(query.name.as_deref(), query.value.as_deref())
            .map(Json)
            .map_err(Into::into)
    })
    .await?
}

async fn tag_create(
    State(cindy): State<Cindy>,
    Path((name, value)): Path<(String, String)>,
) -> Result<(), Error> {
    let database = cindy.database().await;
    spawn_blocking(move || database.tag_add(&name, &value).map_err(Into::into)).await?
}

async fn tag_delete(
    State(cindy): State<Cindy>,
    Query(query): Query<TagQuery<String>>,
) -> Result<(), Error> {
    let database = cindy.database().await;
    spawn_blocking(move || {
        database
            .tag_delete(query.name.as_deref(), query.value.as_deref())
            .map_err(Into::into)
    })
    .await?
}

pub fn router() -> Router<Cindy> {
    Router::new()
        .route("/", get(tag_list).delete(tag_delete))
        .route("/:name/:value", post(tag_create))
        .route("/names", get(tag_names))
}
