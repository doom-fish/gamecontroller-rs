use std::fmt;

/// Errors produced by the JSON-backed `GameController` bridge helpers.
#[derive(Debug)]
pub enum GameControllerError {
    /// The Swift bridge returned a null pointer instead of a JSON payload.
    NullBridgeResponse,
    /// The Swift bridge returned bytes that were not valid UTF-8.
    InvalidUtf8(std::str::Utf8Error),
    /// The JSON payload could not be decoded into the requested Rust type.
    Json(serde_json::Error),
}

impl fmt::Display for GameControllerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NullBridgeResponse => f.write_str("GameController bridge returned null"),
            Self::InvalidUtf8(err) => {
                write!(f, "GameController bridge returned invalid UTF-8: {err}")
            }
            Self::Json(err) => write!(f, "GameController bridge returned invalid JSON: {err}"),
        }
    }
}

impl std::error::Error for GameControllerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NullBridgeResponse => None,
            Self::InvalidUtf8(err) => Some(err),
            Self::Json(err) => Some(err),
        }
    }
}

impl From<std::str::Utf8Error> for GameControllerError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::InvalidUtf8(value)
    }
}

impl From<serde_json::Error> for GameControllerError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}
