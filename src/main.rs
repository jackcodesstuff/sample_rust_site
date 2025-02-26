#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref VISITOR_COUNT: Mutex<u32> = Mutex::new(0);
}

#[get("/")]
fn index() -> Template {
    let mut count = VISITOR_COUNT.lock().unwrap();
    *count += 1;

    let count_string = count.to_string(); // Create a longer-lived string
    let mut context = HashMap::new();
    context.insert("message", "Hello, world!");
    context.insert("count", &count_string); // Borrow the string slice
    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}