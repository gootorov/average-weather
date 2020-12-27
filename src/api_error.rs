use serde::{Serialize, Serializer};
use std::fmt;

/// Api Error type.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ApiError {
    /// Indicates the origin of the error, e.g. OpenWeatherMap, WeatherBit, etc.
    origin: &'static str,
    /// Indicates the type of the error.
    kind: ErrorKind,
}

/// Error variants.
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    /// The error if the data source was not able to provide
    /// weather forecast for the given location.
    InvalidLocation,
    /// The error if the data source returns "invalid api key" type of error.
    /// Not sure if the clients of the API should know about this.
    InvalidApiKey,
    /// The error if we were not able to connect to the data source.
    FailedConnection,
    /// The error if we were not able to parse the returned JSON.
    InvalidJSON,
}

impl ApiError {
    pub fn new(origin: &'static str, kind: ErrorKind) -> Self {
        Self { origin, kind }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::InvalidLocation => write!(f, "Requested location is invalid or unknown."),
            ErrorKind::InvalidApiKey => write!(f, "Api key for the given data source is invalid."),
            ErrorKind::FailedConnection => write!(f, "Failed connecting to the data source."),
            ErrorKind::InvalidJSON => write!(f, "Could not parse returned JSON."),
        }
    }
}

impl Serialize for ErrorKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&*self.to_string())
    }
}
