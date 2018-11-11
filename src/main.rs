#![feature(proc_macro_hygiene, decl_macro)]

extern crate e_vent_server;
use e_vent_server::*;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "This is E-vent's backend server. You probably want to go to e-vent.github.io"
}

fn main() {
    rocket::ignite()
        .manage(EventBackend::new())
        .mount("/", routes![index])
        .launch();
}