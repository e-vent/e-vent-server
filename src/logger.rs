use super::{Event, RawEvent};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};

pub struct IPLogger();

impl Fairing for IPLogger {
    fn info(&self) -> Info {
        Info {
            name: "E-vent Server IP Logger",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        if let Some(remote) = request.remote() {
            println!("Request addr: {}", remote);
        } else {
            println!("No request addr found?");
        }
    }
}

pub struct UALogger();

impl Fairing for UALogger {
    fn info(&self) -> Info {
        Info {
            name: "E-vent Server User Agent Logger",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        if let Some(ua_header) = request.headers().get_one("User-Agent") {
            println!("User agent: {}", ua_header)
        } else {
            println!("No user agent!")
        }
    }
}

pub fn log_get_ok(id: usize) {
    println!("Got event: {}", id);
}

pub fn log_get_err(id: usize) {
    println!("Event not found: {}", id);
}

pub fn log_count(count: usize) {
    println!("Counted: {}", count);
}

pub fn log_too_many_events() {
    println!("Too many events!");
}

pub fn log_add_event(event: &Event) {
    println!("Adding event: {:?}", event)
}

pub fn log_post_ok(id: usize) {
    println!("Event added: {}", id);
}

pub fn log_post_add_err(raw_event: &RawEvent) {
    println!("Could not add event: {:?}", raw_event);
}

pub fn log_post_validate_err(raw_event: &RawEvent) {
    println!("Could not validate event: {:?}", raw_event);
}
