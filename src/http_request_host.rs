use rocket::http::Status;
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use std;

pub struct HttpRequestHost {
    pub origin: String,
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
