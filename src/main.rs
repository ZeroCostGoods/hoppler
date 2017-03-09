#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate dotenv;
extern crate rocket;
extern crate serde_json;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use rocket_contrib::{JSON, Value};
use rocket::http::Status;
use rocket::Outcome;
use rocket::response::{self, Response, Responder};
use rocket::request::{self, Request, FromRequest};
use rocket::State;
use std::env;
use std::sync::Mutex;

mod http_request_host;
pub mod hopplerdb;
pub mod models;
pub mod schema;

use http_request_host::HttpRequestHost;
use models::{JsonEvent, DbEvent, NewDbEvent};

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
//    use schema::events::dsl::*;
//
//    let mut db = db.inner().lock().unwrap();
//    let results = events.filter(in_focus.eq(true))
//        .load::<models::DbEvent>(&*db)
//        .expect("Error loading events");
//
//    for event in results {
//        println!("{} {} {}", event.timestamp, event.hostname, event.pathname)
//    }

    format!("Done")
}

#[post("/events", data = "<events>")]
fn post_events<'a>(db: State<DbConn>, events: JSON<models::EventsList>, req_host: HttpRequestHost) -> Response<'a> {
    use schema::events;

    let mut db = db.inner().lock().unwrap();
    for event in events.events.iter() {
        let new_event = NewDbEvent {
            timestamp: event.timestamp
        };

        diesel::insert(&new_event).into(events::table)
            .execute(&*db)
            .expect("Error saving new event");
        println!("{}", event.timestamp);
    }

    Response::build()
        .raw_header("access-control-allow-credentials", "true")
        .raw_header("Access-Control-Allow-Origin", req_host.origin)
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, POST")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .finalize()
}

fn main() {
//    use schema::events::dsl::*;

    let dbconnection = hopplerdb::establish_connection();

//    let results = events.filter(in_focus.eq(true))
//        .load::<models::Event>(&dbconnection)
//        .expect("Error loading events");
    rocket::ignite().manage(Mutex::new(dbconnection)).mount("/", routes![index, options_handler, post_events]).launch();
}