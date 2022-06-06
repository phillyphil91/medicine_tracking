#[macro_use]
extern crate diesel;

mod common_structs;
mod error_custom;
mod medicine_logic;
mod model;
mod postgres_custom;
mod schema;

use common_structs::QueryDataResponse;
use error_custom::*;
use medicine_logic::return_recommended_dosage_and_count;
use postgres_custom::{dosage_to_postgres, query_data};

use simplelog::*;
use std::fs::File;

use std::sync::Once;
static INIT: Once = Once::new();

#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::serde::json::Json;

// #[cfg(test)]
// mod tests;

#[derive(Responder)]
#[response(status = 500, content_type = "text")]
struct My500Responder {
    inner: String,
}

#[get("/dosage")]
async fn get_dosage() -> Result<Json<QueryDataResponse>, My500Responder> {
    let current_dosage = query_data().await;
    match current_dosage {
        Ok(x) => Ok(Json(x)),
        Err(_e) => Err(My500Responder {
            inner: "Couldn't retrieve current dosage.".to_string(),
        }),
    }
}

#[get("/recommended_dosage")]
async fn get_recommended_dosage() -> Result<Json<QueryDataResponse>, My500Responder> {
    match query_data().await {
        Ok(current_dosage) => match return_recommended_dosage_and_count(&current_dosage) {
            Ok(x) => Ok(Json(x)),
            Err(_e) => Err(My500Responder {
                inner: "No recommended dosage available".to_string(),
            }),
        },
        Err(_e) => Err(My500Responder {
                inner: "No recommended dosage available".to_string(),
            }),
    }
}

#[post("/dosage", data = "<dosage>")]
async fn set_dosage(dosage: String) -> Result<String, My500Responder> {
    match dosage_to_postgres(dosage).await {
        Ok(_x) => Ok("Insert worked".to_string()),
        Err(_e) => Err(My500Responder {
            inner: "Insert didn't work.".to_string(),
        }),
    }
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
        .mount("/", routes![get_dosage, set_dosage, get_recommended_dosage])
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
    fn test_get_recommended_dosage_body() {
        let client = Client::tracked(rocket()).expect("Invalid rocket instance");
        let response = client.get("/recommended_dosage").dispatch();

        assert_eq!(
            response.into_string().unwrap(),
            "{\"dosage\":1.25,\"dosage_count\":0}"
        );
    }
    #[test]
    fn test_set_recommended_dosage_status() {
        let client = Client::tracked(rocket()).expect("Invalid rocket instance");
        let response = client.post("/dosage").body("1.25").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_set_recommended_dosage_body() {
        let client = Client::tracked(rocket()).expect("Invalid rocket instance");
        let response = client.post("/dosage").body("1.25").dispatch();

        assert_eq!(response.into_string().unwrap(), "Insert worked");
    }

    #[test]
    fn test_set_wrong_dosage_body() {
        let client = Client::tracked(rocket()).expect("Invalid rocket instance");
        let response = client.post("/dosage").body("5.0").dispatch();

        assert_eq!(response.into_string().unwrap(), "Insert didn't work.");
    }
}
