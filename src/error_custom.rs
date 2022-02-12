use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};

use std::fmt::{Debug, Display};
use tokio_postgres::error::Error as postgres_error_official;

pub enum CustomError {
    PostgresError(postgres_error_official),
    StringToFloatError(std::num::ParseFloatError),
}

impl std::error::Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::PostgresError(postgres_error) => write!(f, "{}", postgres_error),
            CustomError::StringToFloatError(parse_error) => write!(f, "{}", parse_error),
        }
    }
}
impl Debug for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::PostgresError(postgres_error) => {
                write!(f, "{}", postgres_error.to_string())
            }
            CustomError::StringToFloatError(parse_error) => {
                write!(f, "{}", parse_error.to_string())
            }
        }
    }
}
impl From<postgres_error_official> for CustomError {
    fn from(err: postgres_error_official) -> Self {
        CustomError::PostgresError(err)
    }
}
impl From<std::num::ParseFloatError> for CustomError {
    fn from(err: std::num::ParseFloatError) -> Self {
        CustomError::StringToFloatError(err)
    }
}
impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, req: &'r Request) -> response::Result<'static> {
        match self {
            CustomError::PostgresError(x) => {
                println!("{}", x);
                Status::BadRequest.respond_to(req)
            }
            CustomError::StringToFloatError(x) => {
                println!("{}", x);
                Status::BadRequest.respond_to(req)
            } // CustomError::PostgresError(_) => {
              //     let body = "Database Error";

              //     Response::build()
              //         .status(Status::BadRequest)
              //         .sized_body(body.len(), Cursor::new(body))
              //         .ok()
              // }
        }
    }
}
