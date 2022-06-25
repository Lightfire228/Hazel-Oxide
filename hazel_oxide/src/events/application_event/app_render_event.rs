
use crate::events::event::*;
use crate::events::event::EventCategory as ec;

struct AppRenderEvent {
    pub event: Event,
}

impl AppRenderEvent {
    pub fn new() -> AppRenderEvent {

        let category_flags = ec::EVENT_CATEGORY_APPLICATION.bits();

        AppRenderEvent {
            event: Event::new(EventType::AppRender, "AppRenderEvent", category_flags),
        }
    }
}

impl ToString for AppRenderEvent {
    fn to_string(&self) -> String {
        format!("AppRenderEvent")
    }
}
