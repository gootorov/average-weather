#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(test)] mod tests;
mod api_error;
mod api_response;
mod data_source;
mod weather_data;

use api_error::ApiError;
use api_response::{ApiResponse, Status};
use rocket::{get, routes, State};
use rocket_contrib::json::Json;
use data_source::{DataSource, WeatherBit};
use itertools::{Itertools, Either};
use weather_data::WeatherData;

const INDEX: &str = "Hello, World\n";

type DataSources = [Box<dyn DataSource + Send + Sync>; 1];
fn get_data_sources() -> DataSources {
    [Box::new(WeatherBit::from_envvar())]
}

fn compute_average_data<T>(responses: T) -> Json<ApiResponse>
    where T: Iterator<Item = Result<Vec<WeatherData>, ApiError>>
{
    // this is very flexible, as it allows to add/remove/change
    // data sources arbitrarily at the cost of a bit
    // of code complexity, and i'm not sure if it is worth it.
    let (data, errors): (Vec<_>, Vec<_>) = responses
        .into_iter()
        .partition_map(|r| match r {
            Ok(data) => Either::Left(data),
            Err(e) => Either::Right(e)
        });

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

    match average_data {
        Some(data) => Json(ApiResponse::new(Status::Success, data, errors)),
        None => Json(ApiResponse::new(Status::Fail, vec![], errors))
    }
}

#[get("/forecast/today/<location>")]
fn forecast_today(location: String, sources: State<DataSources>) -> Json<ApiResponse> {
    let responses = sources.iter().map(|source| source.forecast_today(&location));
    compute_average_data(responses)
}

#[get("/forecast/tomorrow/<location>")]
fn forecast_tomorrow(location: String, sources: State<DataSources>) -> Json<ApiResponse> {
    let responses = sources.iter().map(|source| source.forecast_tomorrow(&location));
    compute_average_data(responses)
}

#[get("/forecast/five-days/<location>")]
fn forecast_5_days(location: String, sources: State<DataSources>) -> Json<ApiResponse> {
    let responses = sources.iter().map(|source| source.forecast_5_days(&location));
    compute_average_data(responses)
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
