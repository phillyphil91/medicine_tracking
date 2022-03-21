#[macro_use]
extern crate diesel;

mod error_custom;
mod model;
mod postgres_custom;
mod schema;

use error_custom::CustomError;
use postgres_custom::{struct_to_postgres, struct_to_postgres_diesel};

use log::info;
use simplelog::*;
use std::fs::File;

use std::sync::Once;
static INIT: Once = Once::new();

#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::FileServer;

#[cfg(test)]
mod tests;

#[post("/set_dosage", data = "<dosage>")]
async fn set_dosage(dosage: Option<Form<String>>) -> Result<String, CustomError> {
    match dosage {
        Some(x) => {
            info!("Dosage recorded: {}", x.to_string());
            Ok(struct_to_postgres(x.to_string()).await?)
        }
        None => {
            info!("No dosage recorded");
            Ok(("No Dosage set. Doing nothing :(").to_string())
        }
    }
}

#[post("/set_dosage_diesel", data = "<dosage>")]
async fn set_dosage_diesel(dosage: Option<Form<String>>) -> Result<String, CustomError> {
    match dosage {
        Some(x) => {
            info!("Dosage recorded: {}", x.to_string());
            Ok(struct_to_postgres_diesel(x.to_string()).await?)
        }
        None => {
            info!("No dosage recorded");
            Ok(("No Dosage set. Doing nothing :(").to_string())
        }
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
        .mount("/", routes![set_dosage, set_dosage_diesel])
}
