#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::serde::{json::Json, Serialize};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref VISITOR_COUNT: Mutex<u32> = Mutex::new(0);
}

#[derive(Serialize)]
struct Count {
    count: u32,
}

#[get("/count")]
fn count() -> Json<Count> {
    let mut count = VISITOR_COUNT.lock().unwrap();
    *count += 1;
    Json(Count { count: *count })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![count])
}