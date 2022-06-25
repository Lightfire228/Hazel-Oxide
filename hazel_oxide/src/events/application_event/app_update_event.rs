
use crate::events::event::*;
use crate::events::event::EventCategory as ec;

struct AppUpdateEvent {
    pub event: Event,
}

impl AppUpdateEvent {
    pub fn new() -> AppUpdateEvent {

        let category_flags = ec::EVENT_CATEGORY_APPLICATION.bits();

        AppUpdateEvent {
            event: Event::new(EventType::AppUpdate, "AppUpdateEvent", category_flags),
        }
    }
}

impl ToString for AppUpdateEvent {
    fn to_string(&self) -> String {
        format!("AppUpdateEvent")
    }
}
