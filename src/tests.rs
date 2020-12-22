use super::INDEX;
use super::rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn test_index() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(INDEX.into()));
}
