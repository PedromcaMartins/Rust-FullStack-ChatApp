use rocket::{get, routes, Route};

mod message;

pub fn routes() -> Vec<Route> {
    routes![
        index, 
        message::get_messages, message::post_message
    ]
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}