#[macro_use]
extern crate log;
extern crate env_logger;

extern crate iron;
extern crate router;
extern crate params;
extern crate palette;

// To run, $ cargo run --example simple
// To use, go to http://localhost:3000/test and see output "test"
// Or, go to http://localhost:3000 to see a default "OK"

use iron::{Iron, Request, Response, IronResult, Plugin};
use iron::status;
use router::Router;
use params::{Params, Value};
use std::process::Command;
use palette::named;

fn main() {
    env_logger::init().unwrap();

    println!("starting up");

    let mut router = Router::new();
    router.get("/", handler, "handler");
    router.get("/lights", lights_handler, "lights_handler");
    router.get("/:query", query_handler, "query_handler");

    Iron::new(router).http("0.0.0.0:8888").unwrap();

    fn handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "OK")))
    }

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions
            .get::<Router>()
            .unwrap()
            .find("query")
            .unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }

    fn lights_handler(req: &mut Request) -> IronResult<Response> {
        // let ref url = req.url;
        // println!("URL: {:?}", url);

        let ref params = req.get_ref::<Params>().unwrap();
        let param = params.find(&["q"]).unwrap();
        let query = if let &Value::String(ref x) = param {
            &x
        } else {
            ""
        };

        println!("lights {}", query);

        let (r, g, b) = match query {
            "off" => named::BLACK,
            "on" => named::WHITE,
            "0" => named::BLACK,
            "10" => named::WHITE,
            "false" => named::BLACK,
            "true" => named::WHITE,
            q => named::from_str(q).unwrap_or(named::WHITE),
        };

        Command::new("python")
            .args(&["/home/pi/Pimoroni/mote/examples/rgb.py",
                    &format!("{}", r),
                    &format!("{}", g),
                    &format!("{}", b)])
            .output()
            .expect("failed to execute process");

        Ok(Response::with((status::Ok, "OK")))
    }
}
