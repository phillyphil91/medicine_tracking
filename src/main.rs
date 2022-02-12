//TODO: implement logging

mod error_custom;
mod postgres_custom;

use error_custom::CustomError;
use postgres_custom::struct_to_postgres;

#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::FileServer;

#[cfg(test)]
mod tests;

#[post("/set_dosage?", data = "<dosage>")]
async fn set_dosage(dosage: Option<Form<String>>) -> Result<String, CustomError> {
    match dosage {
        Some(x) => Ok(struct_to_postgres(x.to_string()).await?),
        None => Ok(format!("No Dosage set. Doing nothing :(")),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("./static"))
        .mount("/", routes![set_dosage])
}
