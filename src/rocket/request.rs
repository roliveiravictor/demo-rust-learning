#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::{Future};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

#[get("/forwarding")]
fn forwarding() -> &'static str {
    // Core is the Tokio event loop used for making a non-blocking request
    let mut core = Core::new().unwrap();

    let client = Client::new(&core.handle());

    let url : Uri = "http://httpbin.org/response-headers?foo=bar".parse().unwrap();
    assert_eq!(url.query(), Some("foo=bar"));

    let request = client.get(url)
        .map(|res| {
            assert_eq!(res.status(), hyper::Ok);
        });

    // request is a Future, futures are lazy, so must explicitly run
    core.run(request).unwrap();

    "Forwarded!"
}

fn main() {
    rocket::ignite().mount("/", routes![forwarding]).launch();
}