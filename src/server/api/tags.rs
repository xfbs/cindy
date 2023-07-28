use crate::{server::Error, Cindy};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use cindy_common::tag::{Tag, TagNameInfo};
use std::collections::{BTreeMap, BTreeSet};
use tokio::task::spawn_blocking;

async fn tag_names(
    State(cindy): State<Cindy>,
) -> Result<Json<BTreeMap<String, TagNameInfo>>, Error> {
    let database = cindy.database().await;
    spawn_blocking(move || database.tag_names().map(Json).map_err(Into::into)).await?
}

async fn tag_list(
    State(cindy): State<Cindy>,
    Path((name, value)): Path<(String, String)>,
) -> Result<Json<BTreeSet<Tag>>, Error> {
    let database = cindy.database().await;
    spawn_blocking(move || {
        database
            .tag_list(Some(&name), Some(&value))
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
    Path((name, value)): Path<(String, String)>,
) -> Result<(), Error> {
    let database = cindy.database().await;
    spawn_blocking(move || {
        database
            .tag_delete(Some(&name), Some(&value))
            .map_err(Into::into)
    })
    .await?
}

pub fn router() -> Router<Cindy> {
    Router::new()
        .route(
            "/:name/:value",
            get(tag_list).post(tag_create).delete(tag_delete),
        )
        .route("/names", get(tag_names))
}
