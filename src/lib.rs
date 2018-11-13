
#[macro_use] extern crate serde_derive;

use std::sync::{Arc, Mutex};

mod logger;
pub use self::logger::*;
mod error_obfuscator;
pub use self::error_obfuscator::*;
mod allow_cors;
pub use self::allow_cors::*;

static MAX_EVENTS: usize = 1000;
static VALID_BGS: &'static [&'static str] = &[
    "desmarais",
    "minto",
    "tabaret",
];

#[derive(Clone, Deserialize, Debug)]
pub struct RawEvent {
    pub name: String,
    pub desc: String,
    pub bg: String,
}

#[derive(PartialEq, Eq, Serialize, Clone, Debug)]
pub struct Event {
    name: String,
    desc: String,
    bg: String,
}

fn validate_chars(s: &str) -> bool {
    for c in s.chars() {
        if c < ' ' || c > '~' { // ASCII control and Unicode are banned
            return false
        }
    }
    true
}

fn validate_details(name: &str, desc: &str, bg: &str) -> bool {
    let name_len = name.len();
    let desc_len = desc.len();
    0 < name_len && name_len < 50 && desc_len <= 280
        && validate_chars(name) && validate_chars(desc) && VALID_BGS.contains(&bg)
}

impl RawEvent {
    pub fn from_details(name: String, desc: String, bg: String) -> RawEvent {
        RawEvent {
            name,
            desc,
            bg,
        }
    }

    pub fn into_validated(self) -> Option<Event> {
        if !validate_details(&self.name, &self.desc, &self.bg) {
            return None;
        }
        Some(Event {
            name: self.name,
            desc: self.desc,
            bg: self.bg,
        })
    }
}

pub struct EventBackend {
    event_list: Arc<Mutex<Vec<Arc<Event>>>>,
}

impl EventBackend {
    pub fn new() -> EventBackend {
        EventBackend {
            event_list: Arc::new(Mutex::new(vec![]))
        }
    }

    pub fn get(&self, id: usize) -> Option<Arc<Event>> {
        if let Some(event) = self.event_list.lock().unwrap().get(id) {
            Some(event.clone())
        } else {
            None
        }
    }

    pub fn add(&self, event: Event) -> Option<usize> {
        let mut events = self.event_list.lock().unwrap();
        if events.len() >= MAX_EVENTS {
            log_too_many_events();
            return None
        }
        log_add_event(&event);
        events.push(Arc::new(event));
        Some(events.len() - 1)
    }

    pub fn count(&self) -> usize {
        self.event_list.lock().unwrap().len()
    }
}
