use std::io::Cursor;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Status};
use rocket::{Request, Response};

pub struct ErrorObfuscator();

impl Fairing for ErrorObfuscator {
    fn info(&self) -> Info {
        Info {
            name: "E-vent Server Error Obfuscator",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _: &Request, response: &mut Response) {
        if response.status() != Status::Ok {
            response.set_status(Status::NotFound);
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new("Error"));
        }
    }
}
