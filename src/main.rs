extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate persistent;
extern crate serde_json;

use persistent::Read;
use iron::prelude::*;
use iron::status;
use router::Router;

mod transpile;

fn main() {
    let mut router = Router::new();
    router.get("/status", status_handler, "status");
    router.post("/transpile", transpile_handler, "transpile");

    let address = "localhost:3001";
    println!("Iron listening on {}", address);
    Iron::new(router).http(address).unwrap();
}

fn status_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "200")))
}

fn transpile_handler(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    return match json_body {
        Ok(Some(json_body)) => Ok(Response::with(transpile::transpile(json_body))),
        Ok(None) => Ok(Response::with("Error: No body in request.")),
        Err(err) => Ok(Response::with(err.to_string()))
    };
}

