use crate::events::key_event::*;

struct KeyPressedEvent {
    pub key_event: KeyEvent,

    repeat_count: u32,

    //TODO: event class type
}

impl KeyPressedEvent {
    fn new(key_code: i32, repeat_count: u32) -> KeyPressedEvent {

        KeyPressedEvent {
            key_event: KeyEvent::new(EventType::KeyPressed, "Key Pressed", key_code),

            repeat_count,
        }
    }
}

impl ToString for KeyPressedEvent {
    fn to_string(&self) -> String {

        let key_code     = self.key_event.key_code;
        let repeat_count = self.repeat_count; 

        format!("KeyPressedEvent: {} ({} repeats)", key_code, repeat_count)
    }
}
