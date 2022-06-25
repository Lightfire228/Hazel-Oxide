use crate::events::event::*;
use crate::events::event::EventCategory as ec;

struct WindowResizedEvent {
    pub event: Event,

    width:  u32,
    height: u32,
}

impl WindowResizedEvent {
    pub fn new(width: u32, height: u32) -> WindowResizedEvent {

        let category_flags = ec::EVENT_CATEGORY_APPLICATION.bits();

        WindowResizedEvent {
            event: Event::new(EventType::WindowClosed, "WindowResizedEvent", category_flags),

            width,
            height,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

impl ToString for WindowResizedEvent {
    fn to_string(&self) -> String {
        format!("WindowResizedEvent: {}, {}", self.width, self.height)
    }
}
