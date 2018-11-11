
use std::sync::{Arc, Mutex};

static VALID_BGS: &'static [&'static str] = &[
    "desmarais",
    "minto",
    "tabaret",
];

#[derive(PartialEq, Eq)]
pub struct Event {
    name: String,
    desc: String,
    bg: &'static str,
}

fn validate_details(name: &str, desc: &str) -> bool {
    let name_len = name.len();
    let desc_len = desc.len();
    0 < name_len && name_len < 20 && desc_len <= 280
}

fn try_coerce_background(bg: &str) -> Option<&'static str> {
    for &x in VALID_BGS {
        if x == bg {
            return Some(x)
        }
    }
    None
}

impl Event {
    pub fn from_details(name: String, desc: String, bg: &str) -> Option<Event> {
        if !validate_details(&name, &desc) {
            return None;
        }
        if let Some(bg) = try_coerce_background(bg) {
            Some(Event {
                name,
                desc,
                bg,
            })
        } else {
            None
        }
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

    pub fn add(&self, event: Event) -> usize {
        let mut events = self.event_list.lock().unwrap();
        events.push(Arc::new(event));
        events.len() - 1
    }
}
