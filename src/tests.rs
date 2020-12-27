use super::rocket;
use super::INDEX;
use crate::data_source::weatherbit::WeatherBit;
use rocket::http::Status;
use rocket::local::Client;
use std::env;

#[test]
fn test_index() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(INDEX.into()));
}
