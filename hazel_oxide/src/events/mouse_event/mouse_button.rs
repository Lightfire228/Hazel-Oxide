use crate::events::event::*;
use crate::events::event::EventCategory as ec;

pub mod mouse_button_pressed;
pub mod mouse_button_released;


struct MouseButtonEvent {
    pub event: Event,

    button: i32,
}

impl MouseButtonEvent {
    pub fn new(event_type: EventType, name: &str, button: i32) -> MouseButtonEvent {

        let category_flags = ec::EVENT_CATEGORY_MOUSE.bits() | ec::EVENT_CATEGORY_INPUT.bits();

        MouseButtonEvent {
            event: Event::new(event_type, name, category_flags),

            button,
        }
    }

    pub fn get_mouse_button(&self) -> i32 {
        self.button
    }
}


