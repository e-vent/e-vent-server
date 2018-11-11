#![feature(proc_macro_hygiene, decl_macro)]

extern crate e_vent_server;
use e_vent_server::*;

#[macro_use] extern crate rocket;
use rocket::State;
extern crate rocket_contrib;
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> &'static str {
    "This is E-vent's backend server. You probably want to go to e-vent.github.io"
}

#[get("/<id>")]
fn get(state: State<EventBackend>, id: usize) -> Option<Json<Event>> {
    if let Some(event) = state.get(id) {
        Some(Json((*event).clone()))
    } else {
        None
    }
}

fn add_dummy_event(state: &EventBackend, name: &str, desc: &str, bg: &str) {
    state.add(Event::from_details(
        String::from(name),
        String::from(desc),
        bg,
    ).unwrap());
}

fn main() {
    let state = EventBackend::new();

    add_dummy_event(&state, "Breathing","The most popular event.", "desmarais");
    add_dummy_event(&state, "Drinking","The second most popular event.", "desmarais");
    add_dummy_event(&state, "Eating","The third most popular event.", "desmarais");

    rocket::ignite()
        .manage(state)
        .mount("/", routes![index, get])
        .launch();
}