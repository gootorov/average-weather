#![feature(proc_macro_hygiene, decl_macro)]

mod api_error;
mod api_response;
mod data_source;
#[cfg(test)]
mod tests;
mod weather_data;

use api_error::ApiError;
use api_response::{ApiResponse, Status};
use data_source::{DataSource, WeatherBit};
use itertools::{Either, Itertools};
use rocket::{get, routes, State};
use rocket_contrib::json::Json;
use weather_data::WeatherData;

const INDEX: &str = "Hello, World\n";

type DataSources = [Box<dyn DataSource + Send + Sync>; 1];
fn get_data_sources() -> DataSources {
    [Box::new(WeatherBit::from_envvar())]
}

/// Partition a sequence of responses from data sources
/// into two parts: a vector of responses from each data source
/// and a vector of failures.
fn partition_data<T>(responses: T) -> (Vec<Vec<WeatherData>>, Vec<ApiError>)
where
    T: Iterator<Item = Result<Vec<WeatherData>, ApiError>>,
{
    let (data, errors) = responses
        .partition_map(|r| match r {
            Ok(data) => Either::Left(data),
            Err(e) => Either::Right(e)
        });

    (data, errors)
}

fn compute_average_data(data: Vec<Vec<WeatherData>>) -> Vec<WeatherData> {
    // sum up data from multiple sources.
    let n_sources = data.len();
    let summed_data = data
        .into_iter()
        .fold1(|summed, next_source_data| {
            summed.into_iter()
                .zip(next_source_data.into_iter())
                .map(|(current, next)| current + next)
                .collect()
        });

    let average_data = summed_data
        .map(|data| {
            data.into_iter()
                .map(|day| day / n_sources as f64)
                .collect()
        });

    average_data.unwrap_or_default()
}

// FIXME: too much boilerplate: three methods differ by one call.
#[get("/forecast/today/<location>")]
fn forecast_today(location: String, sources: State<DataSources>) -> Json<ApiResponse> {
    let responses = sources.iter().map(|source| source.forecast_today(&location));

    let (data, errors) = partition_data(responses);

    let average_data = compute_average_data(data);

    // not sure if status should just be bool,
    // it's unlikely that Status::Error will be useful.
    let status = match average_data.len() == 0 {
        true => Status::Fail,
        false => Status::Success,
    };

    Json(ApiResponse::new(status, average_data, errors))
}

#[get("/forecast/tomorrow/<location>")]
fn forecast_tomorrow(location: String, sources: State<DataSources>) -> Json<ApiResponse> {
    let responses = sources.iter().map(|source| source.forecast_tomorrow(&location));

    let (data, errors) = partition_data(responses);

    let average_data = compute_average_data(data);

    let status = match average_data.len() == 0 {
        true => Status::Fail,
        false => Status::Success,
    };

    Json(ApiResponse::new(status, average_data, errors))
}

#[get("/forecast/five-days/<location>")]
fn forecast_5_days(location: String, sources: State<DataSources>) -> Json<ApiResponse> {
    let responses = sources.iter().map(|source| source.forecast_5_days(&location));

    let (data, errors) = partition_data(responses);

    let average_data = compute_average_data(data);

    let status = match average_data.len() == 0 {
        true => Status::Fail,
        false => Status::Success,
    };

    Json(ApiResponse::new(status, average_data, errors))
}

#[get("/")]
fn index() -> &'static str {
    INDEX
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(get_data_sources())
        .mount("/", routes![index, forecast_today, forecast_tomorrow, forecast_5_days])
}

fn main() {
    env_logger::init();
    rocket().launch();
}
