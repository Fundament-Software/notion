use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Hash)]
#[serde(transparent)]
pub struct StatusCode(u16);

impl StatusCode {
    pub fn code(&self) -> u16 {
        self.0
    }
}

impl Display for StatusCode {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// <https://developers.notion.com/reference/errors>
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
pub struct ErrorResponse {
    pub status: StatusCode,
    pub code: ErrorCode,
    pub message: String,
}

/// <https://developers.notion.com/reference/errors>
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidJson,
    InvalidRequestUrl,
    InvalidRequest,
    ValidationError,
    MissionVersion,
    Unauthorized,
    RestrictedResource,
    ObjectNotFound,
    ConflictError,
    RateLimited,
    InternalServerError,
    ServiceUnavailable,
    #[serde(other)] // serde issue #912
    Unknown,
}

impl Display for ErrorCode {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::error::{ErrorCode, ErrorResponse};

    #[test]
    fn deserialize_error() {
        let error: ErrorResponse = serde_json::from_str(include_str!("tests/error.json")).unwrap();
        assert_eq!(error.code, ErrorCode::ValidationError)
    }

    #[test]
    fn deserialize_unknown_error() {
        let error: ErrorResponse =
            serde_json::from_str(include_str!("tests/unknown_error.json")).unwrap();
        assert_eq!(error.code, ErrorCode::Unknown)
    }
}
