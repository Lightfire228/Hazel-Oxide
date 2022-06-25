
use crate::events::event::*;
use crate::events::event::EventCategory as ec;

struct AppTickEvent {
    pub event: Event,
}

impl AppTickEvent {
    pub fn new() -> AppTickEvent {

        let category_flags = ec::EVENT_CATEGORY_APPLICATION.bits();

        AppTickEvent {
            event: Event::new(EventType::AppTick, "AppTickEvent", category_flags),
        }
    }
}

impl ToString for AppTickEvent {
    fn to_string(&self) -> String {
        format!("AppTickEvent")
    }
}
