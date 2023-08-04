use crate::{server::Error, Cindy};
use axum::{
    extract::{Path, Query, State},
    routing::{get, patch},
    Json, Router,
};
use cindy_common::{
    api::*,
    tag::{Tag, TagNameInfo, TagValueInfo},
};
use std::collections::BTreeMap;
use tokio::task::spawn_blocking;

async fn tag_name_list(
    State(cindy): State<Cindy>,
) -> Result<Json<BTreeMap<String, TagNameInfo>>, Error> {
    let database = cindy.database().await;
    spawn_blocking(move || database.tag_names().map(Json).map_err(Into::into)).await?
}

async fn tag_value_list(
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

async fn tag_value_create(
    State(cindy): State<Cindy>,
    Json(query): Json<TagValueCreateRequest<'static>>,
) -> Result<(), Error> {
    let database = cindy.database().await;
    spawn_blocking(move || {
        database.tag_value_create(&query.name, &query.value)?;
        if let Some(display) = &query.display {
            database.tag_value_display(&query.name, &query.value, display)?;
        }
        Ok(())
    })
    .await?
}

async fn tag_value_delete(
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

async fn tag_name_create(
    State(cindy): State<Cindy>,
    Json(query): Json<TagNameCreateRequest<'static>>,
) -> Result<(), Error> {
    let database = cindy.database().await;
    spawn_blocking(move || {
        database.tag_name_create(&query.name, query.display.as_deref())?;
        Ok(())
    })
    .await?
}

async fn tag_name_edit(
    State(cindy): State<Cindy>,
    Path(name): Path<String>,
    Json(query): Json<TagNameEditRequest<'static>>,
) -> Result<(), Error> {
    let mut database = cindy.database().await;
    spawn_blocking(move || {
        let transaction = database.transaction()?;
        if let Some(display) = &query.display {
            transaction.tag_name_display(&name, &display)?;
        }
        if let Some(name_new) = &query.name {
            transaction.tag_name_rename(&name, &name_new)?;
        }
        transaction.commit()?;
        Ok(())
    })
    .await?
}

pub fn router() -> Router<Cindy> {
    Router::new()
        .route(
            "/tags/values",
            get(tag_value_list)
                .delete(tag_value_delete)
                .post(tag_value_create),
        )
        .route("/tag/:name", patch(tag_name_edit))
        .route("/tags", get(tag_name_list).post(tag_name_create))
}
