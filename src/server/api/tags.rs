use crate::{server::Error, Cindy};
use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use cindy_common::{
    api::*,
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
    Json(query): Json<TagCreateBody<'static>>,
) -> Result<(), Error> {
    let database = cindy.database().await;
    spawn_blocking(move || {
        database.tag_add(&query.name, &query.value)?;
        if let Some(display) = &query.display {
            database.tag_value_display(&query.name, &query.value, display)?;
        }
        Ok(())
    })
    .await?
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
        .route("/values", get(tag_list).delete(tag_delete).post(tag_create))
        .route("/names", get(tag_names))
}
