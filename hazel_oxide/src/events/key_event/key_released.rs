use crate::events::key_event::*;


struct KeyReleasedEvent {
    pub key_event: KeyEvent,

    //TODO: event class type

}

impl KeyReleasedEvent {
    fn new(key_code: i32, repeat_count: u32) -> KeyReleasedEvent {

        KeyReleasedEvent {
            key_event: KeyEvent::new(EventType::KeyReleased, "Key Released", key_code),
        }
    }
}

impl ToString for KeyReleasedEvent {
    fn to_string(&self) -> String {

        let key_code = self.key_event.key_code;

        format!("KeyReleasedEvent: {}", key_code)
    }
}

