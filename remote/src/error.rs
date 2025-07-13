use std::fmt;

#[derive(Debug)]
pub enum WavewatchError {
    RfExplorer(rfe::Error),
    RfExplorerConnection(rfe::ConnectionError),
    Http(reqwest::Error),
    Json(serde_json::Error),
    Io(std::io::Error),
    Config(String),
}

impl fmt::Display for WavewatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WavewatchError::RfExplorer(err) => write!(f, "RF Explorer error: {}", err),
            WavewatchError::RfExplorerConnection(err) => {
                write!(f, "RF Explorer Connection Error: {}", err)
            }
            WavewatchError::Http(err) => write!(f, "HTTP error: {}", err),
            WavewatchError::Json(err) => write!(f, "JSON error: {}", err),
            WavewatchError::Io(err) => write!(f, "IO error: {}", err),
            WavewatchError::Config(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for WavewatchError {}

impl From<reqwest::Error> for WavewatchError {
    fn from(err: reqwest::Error) -> Self {
        WavewatchError::Http(err)
    }
}

impl From<serde_json::Error> for WavewatchError {
    fn from(err: serde_json::Error) -> Self {
        WavewatchError::Json(err)
    }
}

impl From<std::io::Error> for WavewatchError {
    fn from(err: std::io::Error) -> Self {
        WavewatchError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, WavewatchError>;
