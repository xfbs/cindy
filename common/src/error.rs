use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorResponse {
    pub error: String,
    pub cause: Option<Box<ErrorResponse>>,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.error)
    }
}

impl Error for ErrorResponse {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause
            .as_ref()
            .map(|cause| cause as &(dyn Error + 'static))
    }
}

impl ErrorResponse {
    pub fn new(error: &dyn Error) -> Self {
        Self {
            error: error.to_string(),
            cause: error
                .source()
                .map(|error| Box::new(ErrorResponse::new(&error))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_serialize() {
        let error = IoError::new(ErrorKind::NotFound, "not found");
        assert_tokens(
            &ErrorResponse::new(&error),
            &[
                Token::Struct {
                    name: "ErrorResponse",
                    len: 2,
                },
                Token::Str("error"),
                Token::Str("not found"),
                Token::Str("cause"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn can_clone() {
        let response = ErrorResponse {
            error: "Test".into(),
            cause: None,
        };
        let clone = response.clone();
        assert_eq!(clone, response);
        assert_eq!(clone.error, "Test");
        assert!(clone.cause.is_none());
    }

    #[test]
    fn impls_ord_hash() {
        use std::collections::{HashSet, BTreeSet};
        let response = ErrorResponse {
            error: "Test".into(),
            cause: None,
        };
        let other = ErrorResponse {
            error: "New".into(),
            cause: Some(response.clone().into()),
        };
        let _set: HashSet<_> = [response.clone(), other.clone()].into();
        let _set: BTreeSet<_> = [response.clone(), other.clone()].into();
    }

    #[test]
    fn can_debug() {
        let response = ErrorResponse {
            error: "Test".into(),
            cause: None,
        };
        assert!(format!("{response:?}").len() > 0);
    }

    #[test]
    fn can_display() {
        let response = ErrorResponse {
            error: "Test".into(),
            cause: None,
        };
        assert_eq!(format!("{response}"), "Test");
    }

    #[test]
    fn can_error() {
        let root = ErrorResponse {
            error: "Root".into(),
            cause: None,
        };
        let response = ErrorResponse {
            error: "Test".into(),
            cause: Some(root.into()),
        };

        assert_eq!(response.to_string(), "Test");
        assert_eq!(response.source().unwrap().to_string(), "Root");
        assert_eq!(response.source().unwrap().source().is_none(), true);
    }

    #[test]
    fn can_new() {
        let error = anyhow::anyhow!("error").context("something");
        let error_response = ErrorResponse::new(error.as_ref());
        assert_eq!(error_response.to_string(), "something");
        assert_eq!(error_response.source().unwrap().to_string(), "error");
    }
}
