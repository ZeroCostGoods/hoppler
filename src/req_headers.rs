use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

use std::collections::HashMap;

#[derive(Debug)]
pub struct ReqHeaders {
    pub origin: String,
    pub headers: HashMap<String, String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for ReqHeaders {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ReqHeaders, ()> {
        let keys: Vec<_> = request.headers().get("Origin").collect();

        let origin = keys.get(0).map_or("unknown", |ref key| key).into();

        let headers = request.headers().iter().map(|header| {
            (header.name().into(), header.value().into())
        }).collect();

        return Outcome::Success(ReqHeaders { origin: origin, headers: headers });
    }
}
