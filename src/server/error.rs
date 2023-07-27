use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use cindy_common::ErrorResponse;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("waiting for blocking task")]
    Task(#[from] JoinError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("not found")]
    NotFound,
    #[error(transparent)]
    Query(#[from] serde_qs::Error),
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
}

impl Error {
    fn status(&self) -> StatusCode {
        match self {
            Error::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = self.status();
        let response = ErrorResponse::new(&self);
        (status, Json(response)).into_response()
    }
}
