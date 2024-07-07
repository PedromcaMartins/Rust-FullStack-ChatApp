use rocket::{get, routes, Route};

use crate::message;

pub fn routes() -> Vec<Route> {
    routes![
        index, 
        message::routes::get_messages, message::routes::post_message
    ]
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}