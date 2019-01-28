extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate persistent;
extern crate serde_json;
extern crate iron_cors;
extern crate logger;
extern crate simplelog;

#[macro_use]
extern crate log;

use iron::prelude::*;
use iron::status;
use router::Router;
use iron_cors::CorsMiddleware;
use logger::Logger;
use simplelog::*;

mod transpile;

fn main() {
    TermLogger::init(LevelFilter::Info, Config::default());

    let (logger_before, logger_after) = Logger::new(None);
    let cors_middleware = CorsMiddleware::with_allow_any();

    let mut router = Router::new();
    router.get("/status", status_handler, "status");
    router.post("/transpile", transpile_handler, "transpile");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_around(cors_middleware);
    chain.link_after(logger_after);

    let address = "localhost:3001";
    info!("Iron listening on {}", address);
    Iron::new(chain).http(address).unwrap();
}

fn status_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "200")))
}

fn transpile_handler(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    return match json_body {
        Ok(Some(json_body)) => Ok(transpile::transpile(json_body)),
        Ok(None) => Ok(Response::with((status::BadRequest, "Error: No body in request."))),
        Err(err) => Ok(Response::with((status::BadRequest, err.to_string())))
    };
}

