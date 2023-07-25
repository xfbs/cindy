use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    fn description(&self) -> &str {
        &self.error
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause
            .as_ref()
            .map(|cause| cause as &(dyn Error + 'static))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_clone() {
        let response = ErrorResponse {
            error: "Test".into(),
            cause: None,
        };
        let clone = response.clone();
        assert_eq!(clone.error, "Test");
        assert!(clone.cause.is_none());
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
}

