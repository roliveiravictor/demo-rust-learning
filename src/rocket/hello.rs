#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/hello/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/rocket")]
fn rocket() -> &'static str {
    "Firing up!"
}

#[get("/hi/<name>")]
fn hi(name: &RawStr) -> String {
    format!("Hi, {}!", name.as_str())
}

#[get("/heil/<name>/<age>/<cool>")]
fn age_heil(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/user/<id>")]
fn user_int(id: isize) -> String {
    format!("Your id is {}!", id)
}

#[get("/user/<id>", rank = 2)]
fn user(id: &RawStr) -> String {
    format!("Your username is {}!", id)
}

use rocket::http::RawStr;

fn main() {
    rocket::ignite().mount("/", routes![world, rocket, hi, age_heil, user, user_int]).launch();
}