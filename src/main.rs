#![feature(proc_macro_hygiene, decl_macro)]

extern crate e_vent_server;
use e_vent_server::*;

#[macro_use] extern crate rocket;
use rocket::{Rocket, State};
extern crate rocket_contrib;
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> &'static str {
    "This is E-vent's backend server. You probably want to go to e-vent.github.io"
}

#[get("/<id>")]
fn get(state: State<EventBackend>, id: usize) -> Option<Json<Event>> {
    if let Some(event) = state.get(id) {
        log_get_ok(id);
        Some(Json((*event).clone()))
    } else {
        log_get_err(id);
        None
    }
}

#[post("/", format="json", data="<raw_event>")]
fn post(state: State<EventBackend>, raw_event: Json<RawEvent>) -> Option<String> {
    if let Some(event) = raw_event.0.clone().into_validated() {
        if let Some(id) = state.add(event) {
            log_post_ok(id);
            Some(format!("{}", id))
        } else {
            log_post_add_err(&raw_event.0);
            None
        }
    } else {
        log_post_validate_err(&raw_event.0);
        None
    }
}

#[get("/count")]
fn count(state: State<EventBackend>) -> String {
    let count = state.count();
    log_count(count);
    format!("{}", count)
}

fn add_dummy_event(state: &EventBackend, name: &str, desc: &str, bg: &str) {
    state.add(RawEvent::from_details(
        String::from(name),
        String::from(desc),
        String::from(bg),
    ).into_validated().unwrap()).unwrap();
}

fn rocket() -> Rocket {
    let state = EventBackend::new();

    add_dummy_event(&state, "Breathing","The most popular event.", "desmarais");
    add_dummy_event(&state, "Drinking","The second most popular event.", "desmarais");
    add_dummy_event(&state, "Eating","The third most popular event.", "desmarais");

    rocket::ignite()
        .manage(state)
        .attach(IPLogger())
        .attach(ErrorObfuscator())
        .attach(CORSEnabler())
        .mount("/", routes![index, count])
        .mount("/events", routes![get, post])
}

fn main() {
    println!("E-vent backend server running");
    rocket().launch();
}
