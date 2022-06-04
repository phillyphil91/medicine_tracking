#[macro_use]
extern crate diesel;

mod common_structs;
mod error_custom;
mod medicine_logic;
mod model;
mod postgres_custom;
mod schema;

use common_structs::QueryDataResponse;
use error_custom::CustomError;
use medicine_logic::return_recommended_dosage_and_count;
use postgres_custom::query_data;

use log::info;
use simplelog::*;
use std::fs::File;

use std::sync::Once;
static INIT: Once = Once::new();

#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::serde::json::Json;

// #[cfg(test)]
// mod tests;

#[get("/dosage")]
async fn get_dosage() -> Result<Json<QueryDataResponse>, String> {
    let current_dosage = query_data().await;
    match current_dosage {
        Ok(x) => Ok(Json(x)),
        Err(_e) => Err("No dosage to suggest".to_string()),
    }
}

#[get("/recommended_dosage")]
async fn get_recommended_dosage() -> Result<Json<QueryDataResponse>, String> {
    match query_data().await {
        Ok(current_dosage) => match return_recommended_dosage_and_count(&current_dosage) {
            Ok(x) => Ok(Json(x)),
            Err(_e) => Err("No dosage to suggest".to_string()),
        },
        Err(_e) => Err("No dosage to suggest".to_string()),
    }
}

#[post("/dosage", data = "<dosage>")]
async fn set_dosage(dosage: Option<Form<String>>) -> Result<String, CustomError> {
    todo!()
}

#[launch]
fn rocket() -> _ {
    // Necessary to only allow logging to be initialized once
    INIT.call_once(|| {
        WriteLogger::init(
            LevelFilter::Info,
            Config::default(),
            File::create("simple_log.log").unwrap(),
        )
        .unwrap();
    });

    rocket::build()
        .mount("/", FileServer::from("./static"))
        .mount("/", routes![get_dosage, get_recommended_dosage])
}

#[cfg(test)]
mod test_medicine_logic {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn ok_root() {
        let client = Client::tracked(rocket()).expect("Invalid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
    #[test]
    fn test_get_dosage_status_code() {
        let client = Client::tracked(rocket()).expect("Invalid rocket instance");
        let response = client.get("/dosage").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_dosage_status_response_body() {
        let client = Client::tracked(rocket()).expect("Invalid rocket instance");
        let response = client.get("/dosage").dispatch();

        assert_eq!(
            response.into_string().unwrap(),
            "{\"dosage\":1.25,\"dosage_count\":30}"
        );
    }

    #[test]
    fn test_get_recommended_dosage_status_code() {
        let client = Client::tracked(rocket()).expect("Invalid rocket instance");
        let response = client.get("/recommended_dosage").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_recommended_dosage_status_body() {
        let client = Client::tracked(rocket()).expect("Invalid rocket instance");
        let response = client.get("/recommended_dosage").dispatch();

        assert_eq!(
            response.into_string().unwrap(),
            "{\"dosage\":1.25,\"dosage_count\":0}"
        );
    }
}
