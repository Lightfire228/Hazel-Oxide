use crate::events::mouse_event::mouse_button::*;

struct MouseButtonReleasedEvent {
    pub button_event: MouseButtonEvent,

    button_code: i32,

    //TODO: event class type
}

impl MouseButtonReleasedEvent {
    pub fn new(button_code: i32) -> MouseButtonReleasedEvent {

        MouseButtonReleasedEvent {
            button_event: MouseButtonEvent::new(EventType::MouseButtonReleased, "Mouse Button Released", button_code),

            button_code,
        }
    }
}

impl ToString for MouseButtonReleasedEvent {
    fn to_string(&self) -> String {

        format!("MouseReleasedEvent: {}", self.button_code)
    }
}
