//TODO: implement logging

mod error_custom;
mod postgres_custom;

use error_custom::CustomError;
use postgres_custom::struct_to_postgres;

use simplelog::*;
use std::fs::File;
use std::sync::Once;


#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::FileServer;

#[cfg(test)] mod tests;

#[post("/set_dosage", data = "<dosage>")]
async fn set_dosage(dosage: Option<Form<String>>) -> Result<String, CustomError> {
    match dosage {
        Some(x) => Ok(struct_to_postgres(x.to_string()).await?),
        None => Ok(format!("No Dosage set. Doing nothing :(")),
    }
}

#[launch]
fn rocket() -> _ {
    static INIT: Once = Once::new();

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
        .mount("/", routes![set_dosage])
}
