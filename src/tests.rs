#[cfg(test)]
use super::*;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn check_root_ok() {
    let client = Client::tracked(rocket()).expect("Invalid rocket instance");
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn check_set_dosage_without_dosage_ok() {
    let client = Client::tracked(rocket()).expect("Invalid rocket instance");
    let response = client.post("/set_dosage").dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn check_set_dosage() {
    let client = Client::tracked(rocket()).expect("Invalid rocket instance");
    let response = client.post("/set_dosage").dispatch();

    assert_eq!(response.status(), Status::Ok);
}
