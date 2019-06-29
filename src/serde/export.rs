#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde_json;

use std::fs::File;
use std::io::{BufReader, Read};

#[get("/export")]
//fn forwarding (filepath: &str) -> String {
fn export () -> String {
    let file = File::open("assets/test.json")
        .expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };
    contents
}

fn main() {
    rocket::ignite().mount("/", routes![export]).launch();
}