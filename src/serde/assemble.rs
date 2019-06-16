#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
extern crate serde_json;
extern crate serde;

use serde::{Serialize, Deserialize};
use rocket::{get, routes};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[get("/assemble")]
fn assemble() -> String {
    let data = r#" {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let p: Person = serde_json::from_str(data).unwrap();

    format!("Please call {} at the number {}", p.name, p.phones[0])
}

fn main() {
    rocket::ignite().mount("/", routes![assemble]).launch();
}