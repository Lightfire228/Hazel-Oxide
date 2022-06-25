use crate::events::event::*;
use crate::events::event::EventCategory as ec;

struct MouseMovedEvent {
    pub event: Event,

    mouse_x: f64,
    mouse_y: f64,
}

impl MouseMovedEvent {
    pub fn new(mouse_x: f64, mouse_y: f64) -> MouseMovedEvent {

        let category_flags = ec::EVENT_CATEGORY_MOUSE.bits() | ec::EVENT_CATEGORY_INPUT.bits();

        MouseMovedEvent {
            event: Event::new(EventType::MouseMoved, "MouseMovedEvent", category_flags),

            mouse_x,
            mouse_y,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.mouse_x
    }

    pub fn get_y(&self) -> f64 {
        self.mouse_y
    }
}

impl ToString for MouseMovedEvent {
    fn to_string(&self) -> String {
        format!("MouseMovedEvent: {}, {}", self.mouse_x, self.mouse_y)
    }
}
