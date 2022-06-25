
use crate::events::event::*;
use crate::events::event::EventCategory as ec;

struct WindowClosedEvent {
    pub event: Event,
}

impl WindowClosedEvent {
    pub fn new() -> WindowClosedEvent {

        let category_flags = ec::EVENT_CATEGORY_APPLICATION.bits();

        WindowClosedEvent {
            event: Event::new(EventType::WindowClosed, "WindowClosedEvent", category_flags),
        }
    }
}

impl ToString for WindowClosedEvent {
    fn to_string(&self) -> String {
        format!("WindowClosedEvent")
    }
}
