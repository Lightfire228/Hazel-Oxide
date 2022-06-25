use crate::events::event::*;
use crate::events::event::EventCategory as ec;

pub mod key_pressed;
pub mod key_released;


struct KeyEvent {
    pub event: Event,

    key_code: i32,
}

impl KeyEvent {
    fn new(event_type: EventType, name: &str, key_code: i32) -> KeyEvent {

        let category_flags = ec::EVENT_CATEGORY_KEYBOARD.bits() | ec::EVENT_CATEGORY_INPUT.bits();

        KeyEvent {
            event: Event::new(event_type, name, category_flags),

            key_code,
        }
    }

    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }
}


