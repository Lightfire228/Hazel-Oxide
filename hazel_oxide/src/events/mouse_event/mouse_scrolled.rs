use crate::events::event::*;
use crate::events::event::EventCategory as ec;

struct MouseScrolledEvent {
    pub event: Event,

    x_offset: f64,
    y_offset: f64,
}

impl MouseScrolledEvent {
    pub fn new(x_offset: f64, y_offset: f64) -> MouseScrolledEvent {

        let category_flags = ec::EVENT_CATEGORY_MOUSE.bits() | ec::EVENT_CATEGORY_INPUT.bits();

        MouseScrolledEvent {
            event: Event::new(EventType::MouseScrolled, "MouseScrolledEvent", category_flags),

            x_offset,
            y_offset,
        }
    }

    pub fn get_x_offset(&self) -> f64 {
        self.x_offset
    }

    pub fn get_y_offset(&self) -> f64 {
        self.y_offset
    }
}

impl ToString for MouseScrolledEvent {
    fn to_string(&self) -> String {
        format!("MouseScrolledEvent: {}, {}", self.x_offset, self.y_offset)
    }
}
