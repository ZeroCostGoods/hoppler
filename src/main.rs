#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
extern crate serde_yaml;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use rocket_contrib::{JSON};
use rocket::response::{Response};
use rocket::State;
use std::sync::Mutex;
use std::io;

mod options;
mod req_headers;
pub mod hopplerdb;
pub mod models;
pub mod schema;

use options::{PostEventsOptions};
use req_headers::ReqHeaders;
use rocket::response::NamedFile;
use models::{DbEvent};

// Our database mutex so we can pass it to each route handler
type DbConn = Mutex<MysqlConnection>;

#[options("/events")]
fn options_events_handler<'a>(req_headers: ReqHeaders) -> Response<'a> {
    Response::build()
        .raw_header("Access-Control-Allow-Credentials", "true")
        .raw_header("Access-Control-Allow-Origin", req_headers.origin)
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, POST")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .finalize()
}

#[options("/events?<req_options>")]
fn options_events_handler_with_options<'a>(req_headers: ReqHeaders, req_options: PostEventsOptions) -> Response<'a> {
    Response::build()
        .raw_header("Access-Control-Allow-Credentials", "true")
        .raw_header("Access-Control-Allow-Origin", req_headers.origin)
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, POST")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .finalize()
}

#[get("/getjs")]
fn get_hopplerjs() -> io::Result<NamedFile> {
    NamedFile::open("node_modules/hopplerjs/dist/hopplerjs.js")
}

#[get("/events")]
fn get_events(db: State<DbConn>) -> String {
    use schema::events::dsl::*;

    let db = db.inner().lock().unwrap();
    let results = events.load::<DbEvent>(&*db)
        .expect("Error loading events");

    let mut output: String = String::from("{ series:");
    for event in results {
        let json = serde_json::to_string_pretty(&event).unwrap();
        output = output + "[" + &json + "],";
    }

    output = output + "}";

    format!("{}", output)
}

#[post("/events?<req_options>", data = "<events>")]
fn post_events<'a>(db: State<DbConn>, mut events: JSON<models::EventsList>, req_headers: ReqHeaders, req_options: PostEventsOptions) -> Response<'a> {
    use schema::events;

    let mut user_override: Option<&str> = None;

    if let Some(ref header_name) = req_options.uh {
        if let Some(ref header_value) = req_headers.headers.get(header_name) {
            user_override = Some(header_value);
            println!("Have user override {} {}", header_name, header_value);
        }
    }

    let db = db.inner().lock().unwrap();
    for event in events.events.iter_mut() {
        if let Some(username) = user_override {
            event.username = username.into();
        }
        diesel::insert(event)
            .into(events::table)
            .execute(&*db)
            .expect("Error saving new event");
    }

    println!("Wrote {} events to the DB", events.events.len());

    Response::build()
        .raw_header("Access-Control-Allow-Credentials", "true")
        .raw_header("Access-Control-Allow-Origin", req_headers.origin)
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, POST")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .finalize()
}

fn main() {
    let dbconnection = hopplerdb::establish_connection();

    diesel::migrations::run_pending_migrations(&dbconnection)
        .expect("Failed to run DB migrations.");

    rocket::ignite()
        .manage(Mutex::new(dbconnection))
        .mount("/", routes![
            get_hopplerjs,
            options_events_handler,
            options_events_handler_with_options,
            get_events,
            post_events,
        ])
        .launch();
}
