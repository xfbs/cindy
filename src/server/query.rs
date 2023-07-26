use crate::server::Error;
use axum::{
    async_trait,
    extract::{rejection::*, FromRequestParts},
    http::{request::Parts, Uri},
};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Copy, Default)]
pub struct Query<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Self::try_from_uri(&parts.uri)
    }
}

impl<T> Query<T>
where
    T: DeserializeOwned,
{
    pub fn try_from_uri(value: &Uri) -> Result<Self, Error> {
        let query = value.query().unwrap_or_default();
        let params = serde_qs::from_str(query)?;
        Ok(Query(params))
    }
}
