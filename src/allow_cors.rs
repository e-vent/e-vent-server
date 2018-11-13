use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::http::uncased::Uncased;
use rocket::{Request, Response};
use std::borrow::Cow::Borrowed;

pub struct CORSEnabler();

static CORS_HEADER: Header = Header {
    name: Uncased { string: Borrowed("Access-Control-Allow-Origin") },
    value: Borrowed("*"),
};

impl Fairing for CORSEnabler {
    fn info(&self) -> Info {
        Info {
            name: "E-vent Server Cross-Origin Sharing Enabler",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _: &Request, response: &mut Response) {
        response.set_header(CORS_HEADER.clone());
    }
}
