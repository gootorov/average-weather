use crate::api_error::ApiError;
use crate::weather_data::WeatherData;
use serde::Serialize;

/// Api Response.
/// Contains a response status, data, and possible errors.
#[derive(Serialize)]
pub struct ApiResponse {
    status: Status,
    data: Vec<WeatherData>,
    errors: Vec<ApiError>,
}

/// Api Response Status.
#[derive(Serialize)]
pub enum Status {
    /// Success is returned if at least one data source returns a weather forecast.
    #[serde(rename = "success")]
    Success,
    /// Fail is returned if none of the data sources returned a weather forecast.
    #[serde(rename = "fail")]
    Fail,
}

impl ApiResponse {
    pub fn new(status: Status, data: Vec<WeatherData>, errors: Vec<ApiError>) -> Self {
        Self {
            status,
            data,
            errors,
        }
    }
}
