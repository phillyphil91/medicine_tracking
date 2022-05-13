use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};

use std::fmt::{Debug, Display};

pub enum CustomError {
    DosageNotRecommend,
}

impl std::error::Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::DosageNotRecommend => {
                write!(f, "The given dosage is not the recommended one")
            }
        }
    }
}
impl Debug for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::DosageNotRecommend => {
                write!(f, "The given dosage is not the recommended one")
            }
        }
    }
}
impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, req: &'r Request) -> response::Result<'static> {
        match self {
            CustomError::DosageNotRecommend => Status::BadRequest.respond_to(req),
            // CustomError::PostgresError(_) => {
            //     let body = "Database Error";

            //     Response::build()
            //         .status(Status::BadRequest)
            //         .sized_body(body.len(), Cursor::new(body))
            //         .ok()
            // }
        }
    }
}
