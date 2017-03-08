#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/hops")]
fn post_hops() -> &'static str {
    "Posted"
}

fn main() {
    rocket::ignite().mount("/", routes![index, post_hops]).launch();
}