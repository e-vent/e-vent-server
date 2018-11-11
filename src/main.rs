#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "This is E-vent's backend server. You probably want to go to e-vent.github.io"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}