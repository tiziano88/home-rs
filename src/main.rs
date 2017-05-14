extern crate iron;
extern crate router;
extern crate params;
extern crate palette;

use iron::{Iron, Request, Response, IronResult, Plugin};
use iron::status;
use router::Router;
use params::{Params, Value};
use std::process::Command;
use palette::named;

fn main() {
    println!("starting up");

    let mut router = Router::new();
    router.get("/", handler, "handler");
    router.get("/lights", get_lights_handler, "lights_handler");
    router.post("/lights", set_lights_handler, "lights_handler");

    Iron::new(router).http("0.0.0.0:8888").unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

fn get_lights_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

fn set_lights_handler(req: &mut Request) -> IronResult<Response> {
    // let ref url = req.url;
    // println!("URL: {:?}", url);

    let ref params = req.get_ref::<Params>().unwrap();
    let param = params.find(&["q"]).unwrap();
    let query = if let &Value::String(ref x) = param {
        x.to_lowercase()
    } else {
        "".to_string()
    };

    println!("lights {}", query);

    let (r, g, b) = match query.as_ref() {
        "off" => named::BLACK,
        "on" => named::WHITE,
        "0" => named::BLACK,
        "10" => named::WHITE,
        "false" => named::BLACK,
        "true" => named::WHITE,
        q => named::from_str(&q).unwrap_or(named::WHITE),
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
