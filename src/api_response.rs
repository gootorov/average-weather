use crate::api_error::ApiError;
use crate::weather_data::WeatherData;
use rocket::http::{self, ContentType};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;
use std::io::Cursor;

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
    pub fn new(data: Vec<WeatherData>, errors: Vec<ApiError>) -> Self {
        // not sure if status should just be bool,
        // it's unlikely that Status::Error will be useful.
        let status = match data.len() {
            0 => Status::Fail,
            _ => Status::Success,
        };

        Self {
            status,
            data,
            errors,
        }
    }
}

impl<'a> Responder<'a> for ApiResponse {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        let body = match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(_) => return Err(http::Status::InternalServerError),
        };

        let status = match self.status {
            Status::Success => http::Status::Ok,
            Status::Fail => http::Status::BadRequest,
        };

        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(Cursor::new(body))
            .ok()
    }
}
