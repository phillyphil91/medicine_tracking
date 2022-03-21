use super::*;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::blocking::Client;
use std::path::Path;

#[test]
fn ok_root() {
    let client = Client::tracked(rocket()).expect("Invalid rocket instance");
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn ok_set_dosage_without_dosage_status() {
    let client = Client::tracked(rocket()).expect("Invalid rocket instance");
    let response = client.post("/set_dosage").dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn ok_set_dosage_without_dosage_body() {
    let client = Client::tracked(rocket()).expect("Invalid rocket instance");
    let response = client.post("/set_dosage").dispatch();
    let response_body = response.into_string();

    assert_eq!(
        response_body,
        Some("No Dosage set. Doing nothing :(".to_string())
    );
}

#[test]
fn ok_set_dosage() {
    // More like an integration test because this will only work with a running postgres db
    let client = Client::tracked(rocket()).expect("Invalid rocket instance");
    let response = client
        .post("/set_dosage")
        .header(ContentType::Form)
        .body("dosage=12")
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
#[test]
fn ok_set_dosage_diesel() {
    // More like an integration test because this will only work with a running postgres db
    let client = Client::tracked(rocket()).expect("Invalid rocket instance");
    let response = client
        .post("/set_dosage_diesel")
        .header(ContentType::Form)
        .body("dosage=25")
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
#[test]
fn error_set_dosage() {
    let client = Client::tracked(rocket()).expect("Invalid rocket instance");
    let response = client
        .post("/set_dosage")
        .header(ContentType::Form)
        .body("dosage=Foo")
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn logging_test() {
    let client = Client::tracked(rocket()).expect("Invalid rocket instance");
    client.get("/").dispatch();

    assert!(Path::new("simple_log.log").exists());
}
