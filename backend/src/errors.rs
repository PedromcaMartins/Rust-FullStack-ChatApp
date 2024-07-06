use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use std::fmt;

#[derive(Debug)]
pub struct CustomError {
    pub message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(Status::InternalServerError)
            .sized_body(self.message.len(), std::io::Cursor::new(self.message))
            .ok()
    }
}
