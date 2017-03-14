#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use rocket_contrib::{JSON};
use rocket::response::{Response};
use rocket::State;
use std::sync::Mutex;

mod http_request_host;
pub mod hopplerdb;
pub mod models;
pub mod schema;

use http_request_host::HttpRequestHost;
use models::{DbEvent};

type DbConn = Mutex<MysqlConnection>;

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
fn index() -> String {
    format!("Hello, world!")
}

#[get("/events")]
fn get_events(db: State<DbConn>) -> String {
    use schema::events::dsl::*;

    let db = db.inner().lock().unwrap();
    let results = events
        .load::<DbEvent>(&*db)
        .expect("Error loading events");

    let mut output:String = String::from("{ series:");
    for event in results {
        let json = serde_json::to_string_pretty(&event).unwrap();
        output = output + "[" + &json + "],";
    }

    output = output + "}";

    format!("{}", output)
}

#[post("/events", data = "<events>")]
fn post_events<'a>(db: State<DbConn>, events: JSON<models::EventsList>, req_host: HttpRequestHost) -> Response<'a> {
    use schema::events;

    let db = db.inner().lock().unwrap();
    for event in events.events.iter() {
        diesel::insert(event).into(events::table)
            .execute(&*db)
            .expect("Error saving new event");
    }

    println!("Wrote {} events to the DB", events.events.len());

    Response::build()
        .raw_header("access-control-allow-credentials", "true")
        .raw_header("Access-Control-Allow-Origin", req_host.origin)
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, POST")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .finalize()
}

fn main() {
    let dbconnection = hopplerdb::establish_connection();
    rocket::ignite().manage(Mutex::new(dbconnection)).mount("/", routes![index, options_handler, get_events, post_events]).launch();
}