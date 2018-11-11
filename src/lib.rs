
#[macro_use] extern crate serde_derive;

use std::sync::{Arc, Mutex};

static VALID_BGS: &'static [&'static str] = &[
    "desmarais",
    "minto",
    "tabaret",
];

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Event {
    name: String,
    desc: String,
    bg: String,
}

fn validate_details(name: &str, desc: &str) -> bool {
    let name_len = name.len();
    let desc_len = desc.len();
    0 < name_len && name_len < 20 && desc_len <= 280
}

fn validate_background(bg: &str) -> bool {
    VALID_BGS.contains(&bg)
}

impl Event {
    pub fn from_details(name: String, desc: String, bg: String) -> Option<Event> {
        if !validate_details(&name, &desc) || !validate_background(&bg) {
            return None;
        }
        Some(Event {
            name,
            desc,
            bg,
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

    pub fn add(&self, event: Event) -> usize {
        let mut events = self.event_list.lock().unwrap();
        events.push(Arc::new(event));
        events.len() - 1
    }

    pub fn count(&self) -> usize {
        self.event_list.lock().unwrap().len()
    }
}
