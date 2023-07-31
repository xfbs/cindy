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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    fn errors() -> Vec<Error> {
        vec![
            Error::NotFound,
            Error::Other(anyhow::anyhow!("Anyhow error")),
        ]
    }

    #[test]
    fn test_response() {
        for error in errors() {
            let status = error.status();
            let response = error.into_response();
            assert_eq!(response.status(), status);
        }
    }

    #[test]
    fn test_debug() {
        for error in errors() {
            println!("{error:?}");
        }
    }

    #[test]
    fn test_status() {
        assert_eq!(Error::NotFound.status(), StatusCode::NOT_FOUND);
        assert_eq!(
            Error::Other(anyhow!("Error")).status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
