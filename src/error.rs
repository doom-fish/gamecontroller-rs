use std::fmt;

#[allow(missing_docs)]
#[derive(Debug)]
pub enum GameControllerError {
    NullBridgeResponse,
    /// The Swift bridge returned bytes that were not valid UTF-8.
    InvalidUtf8(std::str::Utf8Error),
    InvalidBridgeData,
}

impl fmt::Display for GameControllerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NullBridgeResponse => f.write_str("GameController bridge returned null"),
            Self::InvalidUtf8(err) => {
                write!(f, "GameController bridge returned invalid UTF-8: {err}")
            }
            Self::InvalidBridgeData => f.write_str("GameController bridge returned invalid data"),
        }
    }
}

impl std::error::Error for GameControllerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NullBridgeResponse | Self::InvalidBridgeData => None,
            Self::InvalidUtf8(err) => Some(err),
        }
    }
}

impl From<std::str::Utf8Error> for GameControllerError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::InvalidUtf8(value)
    }
}
