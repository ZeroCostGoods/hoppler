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

        let origin;
        if keys.len() != 1 {
            //return Outcome::Failure((Status::BadRequest, ()));
            origin = "unknown".into();
        } else {
            origin = keys[0].to_string();
        }

//        let origin = keys[0].to_string();
        let mut headers = HashMap::new();

        for myhead in request.headers().iter() {
            headers.insert(myhead.name().into(), myhead.value().into());
        }

        return Outcome::Success(ReqHeaders { origin: origin, headers: headers });
    }
}
