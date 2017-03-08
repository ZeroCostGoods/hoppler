#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;

#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::{JSON, Value};
use rocket::http::Status;
use rocket::Outcome;
use rocket::response::{self, Response, Responder};
use rocket::request::{self, Request, FromRequest};

//export interface IEvent {
//    timestamp:number,
//    sessionId:string,
//    timeOnPage:number,
//    username:string,
//    eventType:string,
//    hostname:string,
//    pathname:string
//    search:string,
//    inFocus:boolean,
//    priorHostname?:string,
//    priorPathname?:string,
//    priorSearch?:string,
//}

struct HttpRequestHost {
    origin: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for HttpRequestHost {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<HttpRequestHost, ()> {
        let keys: Vec<_> = request.headers().get("Origin").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];

        return Outcome::Success(HttpRequestHost { origin: key.to_string() });
    }
}

impl std::fmt::Display for HttpRequestHost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.origin)
    }
}

impl std::fmt::Debug for HttpRequestHost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.origin)
    }
}

#[derive(Deserialize)]
struct Event {
    timestamp: u64,
    sessionId: String,
    timeOnPage: u64,
    username: String,
    eventType: String,
    hostname: String,
    pathname: String,
    search: String,
    inFocus: bool,
    priorHostname: String,
    priorPathname: String,
    priorSearch: String
}

#[derive(Deserialize)]
struct Events {
    events: Vec<Event>
}

#[options("/events")]
fn options_handler<'a>(req_host: HttpRequestHost) -> Response<'a> {
    Response::build()
        .raw_header("access-control-allow-credentials", "true")
        .raw_header("Access-Control-Allow-Origin", req_host.origin)
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, POST")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .finalize()
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/events", data = "<events>")]
fn post_events<'a>(events: JSON<Events>, req_host: HttpRequestHost) -> Response<'a> {
    Response::build()
        .raw_header("access-control-allow-credentials", "true")
        .raw_header("Access-Control-Allow-Origin", req_host.origin)
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, POST")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .finalize()
}

fn main() {
    rocket::ignite().mount("/", routes![index, options_handler, post_events]).launch();
}